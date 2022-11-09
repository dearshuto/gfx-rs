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
        assert!(module.entry_points.len() == 1);
        assert!(2 <= module.entry_points[0].operands.len());
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
                let location = module_table.location_table[x.0];
                Attribute { format, location }
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

    location_table: HashMap<u32 /*id*/, u32 /*index*/>,

    input_type_ptr_table: HashMap<u32 /*id */, u32 /*type_id*/>,

    // 定数バッファの id
    constant_buffer_table: Vec<u32>,
}

impl ModuleTable {
    pub fn new(module: &rspirv::dr::Module) -> Self {
        let mut type_table = HashMap::new();
        let mut input_table = HashMap::new(); // input な変数のテーブル
        let mut input_type_ptr_table = HashMap::new();

        for item in &module.types_global_values {
            let id = item.result_id.unwrap();
            match item.class.opcode {
                rspirv::spirv::Op::TypeVector => {
                    assert!(1 <= item.operands.len());
                    let size = match item.operands[1] {
                        rspirv::dr::Operand::LiteralInt32(ref v) => v,
                        _ => panic!(),
                    };
                    match *size {
                        2 => type_table.insert(id, VariableType::Vec2),
                        3 => type_table.insert(id, VariableType::Vec3),
                        _ => None,
                    };
                }
                rspirv::spirv::Op::TypePointer => match item.operands[1] {
                    rspirv::dr::Operand::IdRef(id) => {
                        input_type_ptr_table.insert(item.result_id.unwrap(), id);
                    }
                    _ => (),
                },
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

        // Location の抽出
        let mut location_table = HashMap::new();
        for annotation in &module.annotations {
            // Location データか判定
            let rspirv::dr::Operand::Decoration(d) = annotation.operands[1] else {
                continue;
            };
            if rspirv::spirv::Decoration::Location != d {
                continue;
            }

            // id
            let rspirv::dr::Operand::IdRef(id) = annotation.operands[0] else {
                continue;
            };

            // location
            let rspirv::dr::Operand::LiteralInt32(location) = annotation.operands[2] else {
                continue;
            };

            location_table.insert(id, location);
        }

        // input_table は頂点アトリビュート以外の入力変数も全て入っている
        // そこで entry_point の中で location が設定されているものがアトリビュートと判定する
        let mut attribute_table = HashMap::new();
        for attribute in module.entry_points[0].operands.iter().skip(3) {
            let rspirv::dr::Operand::IdRef( id ) = attribute else {
                continue;
            };

            if let Some((_, target_id)) = location_table.get_key_value(&id) {
                attribute_table.insert(*id, *target_id);
            }
        }

        Self {
            type_table,
            input_table: attribute_table,
            location_table,
            input_type_ptr_table,
            constant_buffer_table: Vec::new(),
        }
    }
}
