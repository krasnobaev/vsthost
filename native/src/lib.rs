#[macro_use]
extern crate neon;
extern crate vst;

use neon::mem::Handle;
use neon::js::JsString;
use neon::vm::{Call, JsResult};

use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

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

// trait CheckArgument<'a> {
//   fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
// }
//
// impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
//   fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
//     self.arguments.require(self.scope, i)?.check::<V>()
//   }
// }

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

register_module!(m, {
  m.export("listAudioDevices", listAudioDevices)?;
  m.export("vstpluginfo", vstpluginfo)?;
  Ok(())
});
