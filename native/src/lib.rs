#[macro_use]
extern crate neon;
extern crate vst;

use neon::mem::Handle;
use neon::js::{JsBoolean, JsString};
use neon::vm::{Call, JsResult};

use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;

extern crate cpal;

/* VST plugin load */

#[allow(dead_code)]
struct SampleHost;

impl Host for SampleHost {
  fn automate(&mut self, index: i32, value: f32) {
    println!("Parameter {} had its value changed to {}", index, value);
  }
}

/**
 * load plugin and show info
 */
fn vstpluginfo(call: Call) -> JsResult<JsString> {
  // let scope = call.scope;
  // call.check_argument::<JsString>(0)?;
  // let filename = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
  let __filename: Handle<JsString> = call.arguments.require(call.scope, 0)?.check::<JsString>()?;
  let filename = String::from(__filename.value());
  let path = Path::new(&filename);

  // Create the host
  let host = Arc::new(Mutex::new(SampleHost));

  println!("Loading {}...", path.to_str().unwrap());

  // Load the plugin
  let mut loader = PluginLoader::load(path, Arc::clone(&host)).unwrap_or_else(
    |e| {
        panic!("Failed to load plugin: {}", e.description())
    },
  );

  // Create an instance of the plugin
  let mut instance = loader.instance().unwrap();

  // Get the plugin information
  let info = instance.get_info();

  let plugininfo = format!(
      "Loaded '{}':\n\t\
       Vendor: {}\n\t\
       Presets: {}\n\t\
       Parameters: {}\n\t\
       VST ID: {}\n\t\
       Version: {}\n\t\
       Initial Delay: {} samples",
      info.name,
      info.vendor,
      info.presets,
      info.parameters,
      info.unique_id,
      info.version,
      info.initial_delay
  );

  // Initialize the instance
  instance.init();
  println!("Initialized instance!");

  // println!("Closing instance...");
  // Close the instance. This is not necessary as the instance is shut down when
  // it is dropped as it goes out of scope.
  // drop(instance);

  Ok(JsString::new(call.scope, &plugininfo).unwrap())
}

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

register_module!(m, {
  m.export("beep", beep)?;
  m.export("listAudioDevices", listAudioDevices)?;
  m.export("vstpluginfo", vstpluginfo)?;
  Ok(())
});
