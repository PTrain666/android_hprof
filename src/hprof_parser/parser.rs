use crate::hprof_parser::snapshot::Snapshot;
use crate::hprof_parser::{constant, ClassRecord, HprofResult, StringRecord};
use crate::{Error, Result};
use std::collections::HashMap;
use std::str;


/// TODO parse
/// complex
/// too many subtag
pub(crate) fn load_heap(snapshot: &mut Snapshot, length: usize) -> Result<bool> {
    let mut cursor = 0;
    while cursor < length {
        // read tag
        let tag = snapshot.read_u8()?;
        cursor += 1;
        match tag {
            constant::ROOT_UNKNOWN => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_JNI_GLOBAL => {
                cursor += load_basic_obj(snapshot)?;
                snapshot.read_id()?;
                cursor += snapshot.id_size();
            }
            constant::ROOT_JNI_LOCAL => {
                cursor += load_jni_local(snapshot)?;
            }
            constant::ROOT_JAVA_FRAME => {
                cursor += load_java_frame(snapshot)?;
            }
            constant::ROOT_NATIVE_STACK => {
                cursor += load_native_stack(snapshot)?;
            }
            constant::ROOT_STICKY_CLASS => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_THREAD_BLOCK => {
                cursor += load_thread_block(snapshot)?;
            }
            constant::ROOT_MONITOR_USED => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_THREAD_OBJECT => {
                cursor += load_thread_obj(snapshot)?;
            }
            constant::CLASS_DUMP => {
                cursor += load_class_dump(snapshot)?;
            }
            constant::INSTANCE_DUMP => {
                cursor += load_instance_dump(snapshot)?;
            }
            constant::OBJECT_ARRAY_DUMP => {
                cursor += load_object_array(snapshot)?;
            }
            constant::PRIMITIVE_ARRAY_DUMP => {
                cursor += load_primitive_array(snapshot)?;
            }
            constant::HEAP_DUMP_INFO => {
                cursor += load_head_dump_info(snapshot)?;
            }
            constant::ROOT_INTERNED_STRING => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_FINALIZING => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_DEBUGGER => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_REFERENCE_CLEANUP => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_VM_INTERNAL => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::ROOT_JNI_MONITOR => {
                cursor += load_basic_obj(snapshot)?;
            }
            constant::HEAP_UNREACHABLE => {
                cursor += load_basic_obj(snapshot)?;
            }
            _ => {
                return Err(Error::UnknownTag(tag));
            }
        }
    }
    return Ok(true);
}

fn load_basic_obj(snapshot: &mut Snapshot) -> Result<usize> {
    let _ = snapshot.read_id()?;
    // println!("load_basic_obj id = {}", root_id);
    return Ok(snapshot.id_size());
}

fn load_jni_local(snapshot: &mut Snapshot) -> Result<usize> {
    let _ = snapshot.read_id()?;
    snapshot.read_u32()?;
    snapshot.read_u32()?;
    // println!("load_jni_local id = {}", root_id);
    return Ok(snapshot.id_size() + 4 * 2);
}

fn load_java_frame(snapshot: &mut Snapshot) -> Result<usize> {
    let _ = snapshot.read_id()?;
    snapshot.read_u32()?;
    snapshot.read_u32()?;
    // println!("load_java_frame id = {}", root_id);
    return Ok(snapshot.id_size() + 4 * 2);
}

fn load_native_stack(snapshot: &mut Snapshot) -> Result<usize> {
    let _ = snapshot.read_id()?;
    snapshot.read_u32()?;
    // println!("load_native_stack id = {}", root_id);
    return Ok(snapshot.id_size() + 4);
}

fn load_thread_block(snapshot: &mut Snapshot) -> Result<usize> {
    let _ = snapshot.read_id()?;
    snapshot.read_u32()?;
    // println!("load_thread_block id = {}", root_id);
    return Ok(snapshot.id_size() + 4);
}

fn load_thread_obj(snapshot: &mut Snapshot) -> Result<usize> {
    let _ = snapshot.read_id()?;
    snapshot.read_u32()?;
    snapshot.read_u32()?;
    // println!("load_java_frame id = {}", root_id);
    return Ok(snapshot.id_size() + 4 * 2);
}

/// TODO parse
/// complex
fn load_class_dump(snapshot: &mut Snapshot) -> Result<usize> {
    // let id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    snapshot.read_u32()?;
    // let class_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    // let classloader_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    snapshot.read_id()?;
    snapshot.read_id()?;
    snapshot.read_id()?;
    snapshot.read_id()?;
    // let instance_size = snapshot.read_u32()?;
    snapshot.read_u32()?;
    let mut bytes_read = (7 * snapshot.id_size()) + 4 + 4;
    // constant pool
    let constant_pool_size = snapshot.read_u16()?;
    bytes_read += 2;
    for _i in 0..constant_pool_size {
        snapshot.read_u16()?;
        bytes_read += 2;
        let field_type = snapshot.read_u8()?;
        bytes_read += 1;
        let size = get_type_size(field_type)?;
        snapshot.read_u8_array(size)?;
        bytes_read += size;
    }
    // static fields
    let static_field_size = snapshot.read_u16()?;
    bytes_read += 2;
    for _i in 0..static_field_size {
        snapshot.read_id()?;
        bytes_read += snapshot.id_size();
        let field_type = snapshot.read_u8()?;
        bytes_read += 1;
        let size = get_type_size(field_type)?;
        snapshot.read_u8_array(size)?;
        bytes_read += size;
    }
    // instance fields
    let instance_field_size = snapshot.read_u16()?;
    bytes_read += 2;
    for _i in 0..instance_field_size {
        snapshot.read_id()?;
        bytes_read += snapshot.id_size();
        // let field_type = snapshot.read_u8()?;
        snapshot.read_u8()?;
        bytes_read += 1;
    }
    // println!("load_class_dump id = {}, class_id = {}, classloader = {}, instance_size = {},\
    // constant_pool_size = {}, static_field_size = {}, instance_field_size = {}",
    //          id, class_id, classloader_id, instance_size,
    //          constant_pool_size, static_field_size, instance_field_size);
    return Ok(bytes_read);
}

/// TODO parse
/// ID | 4byte | ID | remaining
fn load_instance_dump(snapshot: &mut Snapshot) -> Result<usize> {
    // let root_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    snapshot.read_u32()?;
    // let stack_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    let remaining = snapshot.read_u32()? as usize;
    snapshot.read_u8_array(remaining)?;
    // println!("load_instance_dump id = {}", root_id);
    return Ok(snapshot.id_size() * 2 + 4 * 2 + remaining);
}

/// TODO parse
/// ID | 4byte | 4byte(size) |ID | ID*size
fn load_object_array(snapshot: &mut Snapshot) -> Result<usize> {
    // let root_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    // let stack_id = snapshot.read_u32()?;
    snapshot.read_u32()?;
    let size = snapshot.read_u32()? as usize;
    // let class_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    // println!("load_object_array id = {}", root_id);
    let remaining = snapshot.id_size() * size;
    snapshot.read_u8_array(remaining)?;
    return Ok(snapshot.id_size() * 2 + 4 * 2 + remaining);
}

/// TODO parse
/// ID | 4byte | 4byte(size) | 1byte(type) | size*type
fn load_primitive_array(snapshot: &mut Snapshot) -> Result<usize> {
    // let root_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    // let stack_id = snapshot.read_u32()?;
    snapshot.read_u32()?;
    let size = snapshot.read_u32()? as usize;
    let filed_type = snapshot.read_u8()?;
    let type_size = get_type_size(filed_type)?;
    let remaining = size * type_size;
    snapshot.read_u8_array(remaining)?;
    // println!("load_primitive_array id = {}", root_id);
    return Ok(snapshot.id_size() + 4 * 2 + 1 + remaining);
}

/// TODO parse
/// 4 byte | ID
fn load_head_dump_info(snapshot: &mut Snapshot) -> Result<usize> {
    // let heap_id = snapshot.read_u32()?;
    snapshot.read_u32()?;
    // let heap_name_id = snapshot.read_bytes_by_id_size()?;
    snapshot.read_id()?;
    // println!("load_head_dump_info id = {}", heap_id);
    return Ok(snapshot.id_size() + 4);
}

fn get_type_size(field_type: u8) -> Result<usize> {
    let mut size: usize = 0;
    match field_type {
        constant::OBJECT => {
            size += 4;
        }
        constant::BOOLEAN => {
            size += 1;
        }
        constant::CHAR => {
            size += 2;
        }
        constant::FLOAT => {
            size += 4;
        }
        constant::DOUBLE => {
            size += 8;
        }
        constant::BYTE => {
            size += 1;
        }
        constant::SHORT => {
            size += 2;
        }
        constant::INT => {
            size += 4;
        }
        constant::LONG => {
            size += 8;
        }
        _ => {
            return Err(Error::UnknownTag(field_type));
        }
    }
    Ok(size)
}
