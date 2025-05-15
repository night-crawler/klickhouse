use tokio::io::AsyncReadExt;

use crate::{io::ClickhouseRead, values::Value, Result};

use super::{Deserializer, DeserializerState, Type};

pub struct StringDeserializer;

#[allow(clippy::uninit_vec)]
impl Deserializer for StringDeserializer {
    async fn read<R: ClickhouseRead>(
        type_: &Type,
        reader: &mut R,
        rows: usize,
        state: &mut DeserializerState<'_>,
    ) -> Result<Vec<Value>> {
        match type_ {
            Type::String => {
                let mut out = Vec::with_capacity(rows);
                for _ in 0..rows {
                    let data = reader.read_string().await?;
                    let interned = state.intern_bytes_as_maybe_string(data); 
                    out.push(interned);
                }
                Ok(out)
            }
            Type::FixedString(n) => {
                println!("I am fixed!");
                let mut out = Vec::with_capacity(rows);
                for _ in 0..rows {
                    let mut buf = Vec::with_capacity(*n);
                    unsafe { buf.set_len(*n) };
                    reader.read_exact(&mut buf[..]).await?;
                    let first_null = buf.iter().position(|x| *x == 0).unwrap_or(buf.len());
                    buf.truncate(first_null);
                    out.push(state.intern_bytes_as_maybe_string(buf));
                }
                Ok(out)
            }
            _ => unimplemented!(),
        }
    }
}
