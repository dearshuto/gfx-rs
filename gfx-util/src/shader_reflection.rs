use std::collections::HashMap;

use rspirv::{
    dr::Module,
    spirv::{Decoration, Op, StorageClass},
};
use sjgfx_interface::AttributeFormat;

pub struct ShaderReflection {
    pub entry_point: EntryPoint,
    uniform_blocks: Vec<UniformBlock>,
}

impl ShaderReflection {
    pub fn uniform_buffers(&self) -> &[UniformBlock] {
        &self.uniform_blocks
    }
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

pub struct UniformBlock {
    pub binding: u32,
    pub size: usize,
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
            .attributes
            .iter()
            .map(|x| {
                let type_ptr_id = module_table.attribute_type_ptr_table[&x];
                let type_info = &module_table.type_table[&type_ptr_id];
                let format = match type_info {
                    VariableType::Vec2 => AttributeFormat::Float32_32,
                    VariableType::Vec3 => AttributeFormat::Float32_32_32,
                };
                let location = module_table.location_table[x];
                Attribute { format, location }
            })
            .collect();

        let entry_point = EntryPoint {
            name: entry_point_name.clone(),
            attributes,
        };

        let uniform_buffers = Self::reflect_uniform_buffers(module);

        Self {
            entry_point,
            uniform_blocks: uniform_buffers,
        }
    }

    fn reflect_uniform_buffers(module: &rspirv::dr::Module) -> Vec<UniformBlock> {
        let mut uniform_buffers = Vec::new();

        for type_global_value in &module.types_global_values {
            // 変数の id とアノテーションの id が一致しているので、変数型を検索する
            if rspirv::spirv::Op::Variable != type_global_value.class.opcode {
                continue;
            }

            // 変数がUniform かの判定
            // 多分この実装だとテクスチャとかも含んじゃいそう
            let rspirv::dr::Operand::StorageClass(s) = type_global_value.operands[0] else {
                continue;
            };
            if StorageClass::Uniform != s {
                continue;
            }

            // Uniform -> TypePtr
            let Some(result_type) = type_global_value.result_type else {
                continue;
            };
            let type_ptr_id = module.types_global_values.iter().find_map(|x| {
                if x.class.opcode != Op::TypePointer {
                    return None;
                }

                let Some(id) = x.result_id else {
                    return  None;
                };
                if id != result_type {
                    return None;
                }

                if x.operands.len() == 0 {
                    return None;
                }
                let rspirv::dr::Operand::StorageClass(s) = x.operands[0] else {
                    return None;
                };
                if s != rspirv::spirv::StorageClass::Uniform {
                    return None;
                }
                let rspirv::dr::Operand::IdRef(type_ptr_id) = x.operands[1] else {
                    return None;
                };

                return Some(type_ptr_id);
            });
            let Some(type_ptr_id) =type_ptr_id else {
                continue;
            };

            // TypePtr -> TypeStruct
            let type_struct_target_id = module.types_global_values.iter().find_map(|x| {
                // TypeStruct か
                if x.class.opcode != Op::TypeStruct {
                    return None;
                }

                // id が見つけたいやつか
                let Some(id) = x.result_id else {
                    return None;
                };
                if id != type_ptr_id {
                    return None;
                }

                // id の抽出
                if x.operands.len() == 0 {
                    return None;
                }
                let rspirv::dr::Operand::IdRef(target_id) = x.operands[0] else {
                    return None;
                };

                return Some(target_id);
            });
            let Some(type_struct_target_id) = type_struct_target_id else {
                continue;
            };

            //TypeStruct -> (Offset(annotation から抽出), 要素数)
            let offsets: Vec<(u32, u32)> = module
                .annotations
                .iter()
                .filter_map(|x| {
                    if Op::MemberDecorate != x.class.opcode {
                        return None;
                    }

                    let rspirv::dr::Operand::IdRef(annotation_id) = x.operands[0] else {
                     return None;
                   };
                    if annotation_id != type_struct_target_id {
                        return None;
                    }

                    let rspirv::dr::Operand::Decoration(d) = x.operands[2] else {
                    return None;
                };
                    if d != Decoration::Offset {
                        return None;
                    }

                    let rspirv::dr::Operand::LiteralInt32(offset) = x.operands[3] else {
                    return None;
                };

                    let array_size = module.types_global_values.iter().find_map(|x| {
                        if x.class.opcode != Op::TypeStruct {
                            return None;
                        }
                        if x.operands.len() == 0 {
                            return None;
                        }
                        if type_struct_target_id != x.result_id.unwrap() {
                            return None;
                        }
                        let rspirv::dr::Operand::IdRef( target_id) = x.operands[0] else {
                            return None;
                        };
                        
                        // TypeStruct -> TypeArray -> TypeVector
                        let array_count =  module.types_global_values.iter().find_map(|y| {
                            if y.class.opcode != Op::TypeArray {
                                return None;
                            }
                            if y.result_id.unwrap() != target_id {
                                return None;
                            }
                            if y.operands.len() == 0 {
                                return None;
                            }
                            let rspirv::dr::Operand::IdRef( target_vector_id) = y.operands[0] else {
                                return None;
                            };

                            // TypeArray -> TypeVector
                            let array_count = module.types_global_values.iter().find_map(|z| {
                                if z.class.opcode != Op::TypeVector {
                                    return None;
                                }
                                if z.result_id.unwrap() != target_vector_id {
                                    return None;
                                }
                                if z.operands.len() == 0 {
                                    return None;
                                }
                                let rspirv::dr::Operand::LiteralInt32( array_count) = z.operands[1] else {
                                    return None;
                                };
                                return Some(array_count);
                            });

                            return array_count;
                        });
                        return array_count;
                    });

                    if let Some(array_size) = array_size {
                        return Some((offset, array_size));
                    } else {
                        return Some((offset, 1));
                    }
                })
                .collect();

            // 定数バッファのサイズ
            // 定数バッファの最後の変数のオフセットから定数バッファのサイズを算出
            // 16 バイトアラインメントを前提に、最後の変数のオフセットに 16 を足したら定数バッファ全体のサイズになるはず
            let (offset, count) = offsets.last().unwrap();
            let size = (offset + 16/*アラインメント*/) * count;

            let binding = module.annotations.iter().find_map(|x| {
                // id が一致するかの判定
                let rspirv::dr::Operand::IdRef(annotation_id) = x.operands[0] else {
                    return None;
                };
                let id = type_global_value.result_id.unwrap();
                if id != annotation_id {
                    return None;
                }

                // Binding 型かをチェック。参考；DescriptorSet とかがマッチする可能性がある
                let rspirv::dr::Operand::Decoration(d) = x.operands[1] else {
                    return None;
                };
                if d != Decoration::Binding {
                    return None;
                }

                // binding を取得
                let rspirv::dr::Operand::LiteralInt32(binding) = x.operands[2] else {
                    return None;
                };

                return Some(binding);
            });

            let uniform_buffer = UniformBlock {
                binding: binding.unwrap(),
                size: size as usize,
            };
            uniform_buffers.push(uniform_buffer);
        }

        uniform_buffers
    }
}

struct ModuleTable {
    // 型の辞書
    type_table: HashMap<u32, VariableType>,

    // 頂点アトリビュートの id
    attributes: Vec<u32>,

    // 頂点アトリビュートから location を引く辞書
    location_table: HashMap<u32 /*id*/, u32 /*index*/>,

    // 頂点アトリビュートから型ポインタを引く辞書
    attribute_type_ptr_table: HashMap<u32, u32>,
}

impl ModuleTable {
    pub fn new(module: &rspirv::dr::Module) -> Self {
        let mut type_table = HashMap::new();
        let input_table = Self::reflect_inpute_variables(module); // input な変数のテーブル
        let type_ptr_table = Self::reflect_type_ptr(module);
        let location_table = Self::reflect_locations(module);

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
                _ => {}
            }
        }

        // input_table は頂点アトリビュート以外の入力変数も全て入っている
        // そこで entry_point の中で location が設定されているものがアトリビュートと判定する
        let mut attributes = Vec::new();
        for input in &input_table {
            let Some((id, _)) = location_table.get_key_value(&input.0) else {
                continue;
            };
            attributes.push(*id);
        }

        // アトリビュートから型ポインタを引くテーブルを作成
        let mut attribute_type_ptr_table = HashMap::new();
        for attribute_id in &attributes {
            let id = input_table[&attribute_id];
            let value = type_ptr_table[&id];
            attribute_type_ptr_table.insert(*attribute_id, value);
        }

        Self {
            type_table,
            attributes,
            attribute_type_ptr_table,
            location_table,
        }
    }

    fn reflect_locations(module: &Module) -> HashMap<u32, u32> {
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

        location_table
    }

    fn reflect_type_ptr(module: &Module) -> HashMap<u32, u32> {
        let mut result = HashMap::new();
        for item in &module.types_global_values {
            match item.class.opcode {
                rspirv::spirv::Op::TypePointer => match item.operands[1] {
                    rspirv::dr::Operand::IdRef(id) => {
                        result.insert(item.result_id.unwrap(), id);
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        result
    }

    fn reflect_inpute_variables(module: &Module) -> HashMap<u32, u32> {
        let mut input_table = HashMap::new();
        for item in &module.types_global_values {
            match item.class.opcode {
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

        input_table
    }
}
