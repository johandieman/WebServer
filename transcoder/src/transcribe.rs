use pyo3::prelude::*;

pub fn main() -> PyResult<()> {
    
    // pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {

        let whisper = py.import("whisper").unwrap();
        // let editor = py.import("moviepy.editor");
        // let subtitle_tools = py.import("moviepy.video.tools.subtitles");


        let model = whisper.call_method1("load_model", ("base",)).unwrap();

        let audio =whisper.call_method1("load_audio", ("test.mp3",)).unwrap();

        let result = model.call_method1("transcribe", (audio,)).unwrap();



        println!("{:?}", result);    


        Ok(())
    })


}
