use std::collections::HashMap;

use sjgfx_interface::AttributeFormat;

pub struct ShaderReflection {
    pub entry_point: EntryPoint,
}

pub struct EntryPoint {
    name: String,
    attributes: Vec<Attribute>,
}

impl EntryPoint {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attribures(&self) -> &[Attribute] {
        &self.attributes
    }
}

pub struct Attribute {
    format: AttributeFormat,
    location: u32,
}
impl Attribute {
    pub fn format(&self) -> AttributeFormat {
        self.format.clone()
    }

    pub fn location(&self) -> u32 {
        self.location
    }
}

enum VariableType {
    Vec2,
    Vec3,
}

impl ShaderReflection {
    pub fn new_from_biinary(binary: &[u8]) -> Self {
        let module = rspirv::dr::load_bytes(binary).unwrap();
        Self::new_impl(&module)
    }

    fn new_impl(module: &rspirv::dr::Module) -> Self {
        let entry_point_name = match module.entry_points[0].operands[2] {
            rspirv::dr::Operand::LiteralString(ref value) => value,
            _ => panic!(),
        };
        let module_table = ModuleTable::new(module);

        let attributes: Vec<Attribute> = module_table
            .input_table
            .iter()
            .map(|x| {
                let type_ptr_id = module_table.input_type_ptr_table[x.1];
                let type_info = &module_table.type_table[&type_ptr_id];
                let format = match type_info {
                    VariableType::Vec2 => AttributeFormat::Float32_32,
                    VariableType::Vec3 => AttributeFormat::Float32_32_32,
                };
                let index = module_table.input_index_table[x.0];
                Attribute {
                    format,
                    location: index,
                }
            })
            .collect();

        let entry_point = EntryPoint {
            name: entry_point_name.clone(),
            attributes,
        };
        Self { entry_point }
    }
}

struct ModuleTable {
    type_table: HashMap<u32, VariableType>,

    // 頂点アトリビュートのテーブル
    input_table: HashMap<u32 /*id*/, u32 /*type_ptr_id*/>,

    input_index_table: HashMap<u32 /*id*/, u32 /*index*/>,

    input_type_ptr_table: HashMap<u32 /*id */, u32 /*type_id*/>,
}

impl ModuleTable {
    pub fn new(module: &rspirv::dr::Module) -> Self {
        let mut input_index = 0;
        let mut type_table = HashMap::new();
        let mut input_table = HashMap::new();
        let mut input_index_table = HashMap::new();
        let mut input_type_ptr_table = HashMap::new();

        for item in &module.types_global_values {
            let id = item.result_id.unwrap();
            match item.class.opcode {
                rspirv::spirv::Op::TypeVector => {
                    let size = match item.operands[1] {
                        rspirv::dr::Operand::LiteralInt32(ref v) => v,
                        _ => panic!(),
                    };
                    match *size {
                        2 => type_table.insert(id, VariableType::Vec2),
                        3 => type_table.insert(id, VariableType::Vec3),
                        _ => None,
                    };

                    // 頂点アトリビュートのインデクスを保持
                    input_index_table.insert(id, input_index);
                    input_index += 1;
                }
                rspirv::spirv::Op::TypePointer => {
                    input_type_ptr_table.insert(item.result_id.unwrap(), item.result_type.unwrap());
                }
                rspirv::spirv::Op::Variable => {
                    match item.operands[0] {
                        rspirv::dr::Operand::StorageClass(ref class) => match class {
                            rspirv::spirv::StorageClass::Input => {
                                let type_ptr_id = item.result_type.unwrap();
                                input_table.insert(item.result_id.unwrap(), type_ptr_id);
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                }
                _ => {}
            }
        }

        Self {
            type_table,
            input_table,
            input_index_table,
            input_type_ptr_table,
        }
    }
}
