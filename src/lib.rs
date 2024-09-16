use classer::bytecode::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

pub enum Error<'s> {
    InvalidClassFile(&'s str),
}

type CompilerResult<'l, T> = Result<T, Error<'l>>;

#[derive(Debug)]
pub struct Compiler<'l> {
    context: &'l Context,
    module: Module<'l>,
    builder: Builder<'l>,
    class_file: ClassFile,
}

impl<'l> Compiler<'l> {
    pub fn new(class_file: ClassFile, context: &'l Context) -> CompilerResult<'l, Compiler<'l>> {
        let this_class_index = class_file
            .constant_pool
            .get((class_file.this_class - 1) as usize)
            .ok_or(Error::InvalidClassFile("this class not found"))?;
        let this_class = match this_class_index.info {
            CpInfoType::Class { name_index } => {
                let utf8 = class_file
                    .constant_pool
                    .get((name_index - 1) as usize)
                    .ok_or(Error::InvalidClassFile("the name wasnt found"))?;

                match utf8.info {
                    CpInfoType::Utf8 { ref bytes, .. } => bytes,
                    _ => return Err(Error::InvalidClassFile("Invalid class name")),
                }
            }
            _ => return Err(Error::InvalidClassFile("Invalid this class")),
        };
        let module = context.create_module(this_class);
        let builder = context.create_builder();
        Ok(Compiler {
            context,
            module,
            builder,
            class_file,
        })
    }
}
