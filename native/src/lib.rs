#[macro_use]
extern crate neon;
extern crate cpal;
extern crate vst;

use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use std::thread;

use neon::mem::Handle;
use neon::js::{Object, JsBoolean, JsFunction, JsString, JsObject, JsInteger};
use neon::vm::{Call, JsResult};

mod vsthost;

/* CPAL */

fn listAudioDevices(call: Call) -> JsResult<JsString> {
  let line1 = format!("Default Input Device:\n  {:?}", cpal::default_input_device().map(|e| e.name()));
  let line2 = format!("Default Output Device:\n  {:?}", cpal::default_output_device().map(|e| e.name()));

  let devices = cpal::devices();
  let line3 = format!("Devices: ");
  let mut line4forloop = String::new();
  for (device_index, device) in devices.enumerate() {
    line4forloop = format!("{}\n{}. \"{}\"", line4forloop, device_index + 1, device.name());

    // Input formats
    if let Ok(fmt) = device.default_input_format() {
      line4forloop = format!("{}\n  Default input stream format:\n    {:?}", line4forloop, fmt);
    }
    let mut input_formats = match device.supported_input_formats() {
      Ok(f) => f.peekable(),
      Err(e) => {
        line4forloop = format!("{}\nError: {:?}", line4forloop, e);
        continue;
      },
    };
    if input_formats.peek().is_some() {
      line4forloop = format!("{}\n  All supported input stream formats:", line4forloop);
      for (format_index, format) in input_formats.enumerate() {
        line4forloop = format!("{}\n    {}.{}. {:?}", line4forloop, device_index + 1, format_index + 1, format);
      }
    }

    // Output formats
    if let Ok(fmt) = device.default_output_format() {
      line4forloop = format!("{}\n  Default output stream format:\n    {:?}", line4forloop, fmt);
    }
    let mut output_formats = match device.supported_output_formats() {
      Ok(f) => f.peekable(),
      Err(e) => {
        line4forloop = format!("{}\nError: {:?}", line4forloop, e);
        continue;
      },
    };
    if output_formats.peek().is_some() {
      line4forloop = format!("{}\n  All supported output stream formats:", line4forloop);
      for (format_index, format) in output_formats.enumerate() {
        line4forloop = format!("{}\n    {}.{}. {:?}", line4forloop, device_index + 1, format_index + 1, format);
      }
    }
  }

  let res = format!("{}\n{}\n{}{}", line1, line2, line3, line4forloop);
  Ok(JsString::new(call.scope, &res).unwrap())
}

fn beep(call: Call) -> JsResult<JsString> {
  let isrun: Handle<JsBoolean> = call.arguments.require(call.scope, 0)?.check::<JsBoolean>()?;

  if isrun.value() {
    thread::spawn(|| {
      let device = cpal::default_output_device().expect("Failed to get default output device");
      let format = device.default_output_format().expect("Failed to get default output format");
      let event_loop = cpal::EventLoop::new();
      let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
      event_loop.play_stream(stream_id.clone());

      let sample_rate = format.sample_rate.0 as f32;
      let mut sample_clock = 0f32;

      // Produce a sinusoid of maximum amplitude.
      let mut next_value = || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
      };

      event_loop.run(move |_, data| {
        match data {
          cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
              let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
              for out in sample.iter_mut() {
                *out = value;
              }
            }
          },
          cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
              let value = (next_value() * std::i16::MAX as f32) as i16;
              for out in sample.iter_mut() {
                *out = value;
              }
            }
          },
          cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
              let value = next_value();
              for out in sample.iter_mut() {
                *out = value;
              }
            }
          },
          _ => (),
        }
      });
    });
  }

  Ok(JsString::new(call.scope, "").unwrap())
}

/* VST */

static mut LOAD_POINTER: *mut c_void = 0 as *mut c_void;
// fn get_host() -> &'static mut vsthost::VSTHost {
fn get_host() -> &'static Arc<Mutex<vsthost::VSTHost>> {
  unsafe {
    let host = LOAD_POINTER as *const Arc<Mutex<vsthost::VSTHost>>;
    let host = &*host;
    host
    // let mut host = &mut *host.lock().unwrap();
    // &mut *host.lock().unwrap()
  }
}

fn get_vsthost_instance(call: Call) -> JsResult<JsObject> {
  let vst_api = JsObject::new(call.scope);

  unsafe {
    let __vst_host = Arc::new(Mutex::new(vsthost::VSTHost::new()));
    LOAD_POINTER = Box::into_raw(Box::new(Arc::clone(&__vst_host))) as *mut c_void;
  }

  fn loadvstplugin(call: Call) -> JsResult<JsInteger> {
    let __filename: Handle<JsString> = call.arguments.require(call.scope, 0)?.check::<JsString>()?;
    let filename = String::from(__filename.value());

    let index = get_host().lock().unwrap().loadvstplugin(filename) as i32;
    Ok(JsInteger::new(call.scope, index))
  }

  fn vstpluginfo(call: Call) -> JsResult<JsString> {
    let __index: Handle<JsInteger> = call.arguments.require(call.scope, 0)?.check::<JsInteger>()?;
    let index = __index.value() as usize;

    let info = get_host().lock().unwrap().vstpluginfo(index);
    Ok(JsString::new(call.scope, &info).unwrap())
  }

  let _loadvstplugin = JsFunction::new(call.scope, loadvstplugin);
  let _vstpluginfo   = JsFunction::new(call.scope, vstpluginfo);

  vst_api.set("loadvstplugin", _loadvstplugin.unwrap());
  vst_api.set("vstpluginfo", _vstpluginfo.unwrap());

  Ok(vst_api)
}

register_module!(m, {
  m.export("listAudioDevices", listAudioDevices)?;
  m.export("beep", beep)?;
  m.export("get_vsthost_instance", get_vsthost_instance)?;
  Ok(())
});
