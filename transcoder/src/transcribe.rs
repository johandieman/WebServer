use pyo3::{types::{PyByteArray,PyTuple}, prelude::*};

pub fn edit(arr: Vec<u8>) -> PyResult<()> {
    
    // pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {

        let code = std::fs::read_to_string("./src/video.py")?;
        let module = PyModule::from_code(py, &code,"video","video").unwrap();

        let video_class = module.getattr("Video").unwrap();

        let local = arr.as_slice().clone();

        let io_arr = PyByteArray::new(py, local);
        let args = PyTuple::new(py, &[io_arr]);

        let video_object = video_class.call1(args).unwrap();

        let result = video_object.call_method0("getResult").unwrap();

        println!("{:?}", result);    

        Ok(())
    })


}
