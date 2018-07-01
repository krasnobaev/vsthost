use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

// use vst::buffer::SendEventBuffer;
use vst::host::{Host, /*HostBuffer,*/ PluginLoader, PluginInstance};
use vst::plugin::Plugin;

/* VST plugin load */

#[allow(dead_code)]
struct SampleHost;

impl Host for SampleHost {
  fn automate(&mut self, index: i32, value: f32) {
    println!("Parameter {} had its value changed to {}", index, value);
  }
}

pub struct VSTHost {
  host: Arc<Mutex<SampleHost>>,
  plugins: Vec<PluginInstance>,
  pluginfilename: String
}

impl VSTHost {
  pub fn new() -> VSTHost {
    VSTHost {
      host: Arc::new(Mutex::new(SampleHost)),
      plugins: Vec::with_capacity(10),
      pluginfilename: String::new()
    }
  }

  pub fn getpluginfilename(&self) -> &str {
    &self.pluginfilename
  }

  pub fn loadvstplugin(&mut self, filename: String) -> usize {
    self.pluginfilename = filename;
    let path = Path::new(&self.pluginfilename);

    println!("Loading {}...", path.to_str().unwrap());
    let mut loader = PluginLoader::load(path, Arc::clone(&self.host)).unwrap_or_else(
      |e| {
        panic!("Failed to load plugin: {}", e.description())
      },
    );
    let mut instance = loader.instance().unwrap();
    instance.init();
    self.plugins.push(instance);

    println!("Initialized instance!");

    self.plugins.len()
  }

  /**
   * load plugin and show info
   */
  pub fn vstpluginfo(&mut self, index: usize) -> String {
    let info = self.plugins[index].get_info();

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

    plugininfo
  }

  // pub fn send_event_buffer() {
  //   // let mut host_buffer: HostBuffer<f32> = HostBuffer::new(2, 2);
  //   // let inputs = vec![vec![0.0; 1000]; 2];
  //   // let mut outputs = vec![vec![0.0; 1000]; 2];
  //   // let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
  //   // instance.process(&mut audio_buffer);
  //   //
  //   // let mut eb:SendEventBuffer = SendEventBuffer::new(5);
  //   // eb.send_events(, instance);
  // }
}

impl Drop for VSTHost {
  fn drop(&mut self) {
    println!("Drop the Host!");
  }
}
