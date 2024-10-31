use dotenv::dotenv;
use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyTuple},
};
use std::env;

fn call_py_func() -> PyResult<()> {
    Python::with_gil(|py| {
        let def: Py<PyAny> = PyModule::from_code_bound(
            py,
            "
def print_msg(msg='SUP from python', another_msg=None):
    print(msg)
    if another_msg:
        print(f'\t{another_msg}')
            ",
            "",
            "",
        )?
        .getattr("print_msg")?
        .into();

        // Call the python function with no args or kwargs
        def.call0(py)?;

        println!("---");

        // Call the python function with only positional args
        let mut args = PyTuple::new_bound(py, ["SUP from python with ARG from Rust"]);
        def.call1(py, args)?;

        println!("---");

        // Call the python function with positional and keyword args
        args = PyTuple::new_bound(py, ["SUP from python with ARGS and KWARGS from Rust"]);
        let kwargs = [("another_msg", "Sayonara")].into_py_dict_bound(py);
        def.call_bound(py, args, Some(&kwargs))?;

        println!("---");

        Ok(())
    })
}

fn py_load_site_packages(py: Python) -> PyResult<()> {
    let site_packages_path =
        env::var("PYTHON_SITE_PACKAGES").expect("PYTHON_SITE_PACKAGES not set in .env file");
    let sys = py.import_bound("sys")?;
    sys.getattr("path")?
        .call_method1("append", (site_packages_path,))?;
    Ok(())
}

fn py_get_plural_of_noun(noun: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let python_script = include_str!("python_script.py");

        let module = PyModule::from_code_bound(py, python_script, "", "")?;
        let get_plural_noun_func = module.getattr("get_plural_of_noun")?;
        let call = get_plural_noun_func.call1(PyTuple::new_bound(py, [noun]))?;
        let result = match call.extract() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error getting plural noun: {e:?}");
                ""
            }
        };
        Ok(result.to_string())
    })
}

fn py_get_plural_of_verb(verb: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let python_script = include_str!("python_script.py");

        let module = PyModule::from_code_bound(py, python_script, "", "")?;
        let get_plural_verb_func = module.getattr("get_plural_of_verb")?;
        let call = get_plural_verb_func.call1(PyTuple::new_bound(py, [verb]))?;
        let result = match call.extract() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error getting plural verb: {e:?}");
                ""
            }
        };
        Ok(result.to_string())
    })
}

fn main() {
    dotenv().ok();

    // Ensure that the python runner knows where our site-packages are
    Python::with_gil(|py| {
        py_load_site_packages(py).expect("Could not append site-packages to script runner");
    });

    // Call a simple python function with no args, 1 arg, and 1 arg and kwarg
    call_py_func().expect("Wha happen");

    // Call python functions that use a package in a venv
    let singular_noun = "cat";
    let plural_noun = match py_get_plural_of_noun(singular_noun) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            String::from("Could not get plural form")
        }
    };
    println!("Plural of '{singular_noun}': {plural_noun}\n---");

    let singular_verb = "jumps";
    let plural_verb = match py_get_plural_of_verb(singular_verb) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            String::from("Could not get plural form")
        }
    };
    println!("Plural of '{singular_verb}': {plural_verb}");
}
