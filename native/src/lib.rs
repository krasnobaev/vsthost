#[macro_use]
extern crate neon;
extern crate vst;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

use std::sync::{Arc, Mutex};
use std::path::Path;
use std::error::Error;

use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;

/* hello world */

/**
 * send hello world message
 */
fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello from node").unwrap())
}

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
  let scope = call.scope;
  let path = Path::new(
    "/Library/Audio/Plug-Ins/VST/Replika.vst/Contents/MacOS/Replika"
  );

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

  println!("Closing instance...");
  // Close the instance. This is not necessary as the instance is shut down when
  // it is dropped as it goes out of scope.
  // drop(instance);

  Ok(JsString::new(scope, &plugininfo).unwrap())
}

register_module!(m, {
    m.export("hello", hello)?;
    m.export("vstpluginfo", vstpluginfo)?;
    Ok(())
});
