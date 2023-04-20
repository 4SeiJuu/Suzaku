use std::{
    path::PathBuf, 
    fs::{
        self, 
        File
    }, 
    io::Write, 
    collections::HashMap
};

use serde::Deserialize;
use serde_json::Value;
use strum::IntoEnumIterator;

use suzaku_extension_sdk::{
    language::{
        analyzer::{
            LanguageAnalysisResult,
            LanguageAnalysisPolicyError
        }, 
        element::{
            Elements, 
            ElementCategories, 
            IElement, 
            TypeDescriptor, 
            ParamDescriptor, 
            Caller
        }, 
        mapper::{
            LanguageMapResult, 
            LanguageMapPolicy
        }
    },
    utils, 
    ELEMENT_FILE_EXTENSION
};

use crate::java::element::JavaElement;

struct JavaDataMappingListener {
    types: HashMap<String, TypeDescriptor>
}

impl JavaDataMappingListener {
    fn map(&self, vertex: &mut JavaElement) {
        let mapping = |ty: &TypeDescriptor| -> Option<TypeDescriptor> {
            for (name, td) in &self.types {
                if name.as_str() == ty.to_string() || name.ends_with(&ty.to_string()) {
                    return Some(td.clone());
                }

                if let Some(first_name) = ty.name.get(0) {
                    if name.ends_with(first_name) {
                        return Some(td.clone())
                    }
                }
            }
            None
        };

        if let Some(vt) = vertex.get_type() {
            if let Some(new_vt) = match vt {
                // ancestors, annotations, modifiers, name, extends, implements
                Elements::Class(ancestors, annotations, modifiers, name, extends, implements) => {
                    let mut mapped_extends: Vec<TypeDescriptor> = Vec::new();
                    for ori_extend in extends {
                        match mapping(ori_extend) {
                            Some(mapped_extend) => mapped_extends.push(mapped_extend),
                            None => mapped_extends.push(ori_extend.clone())
                        }
                    }

                    let mut mapped_impls: Vec<TypeDescriptor> = Vec::new();
                    for ori_impl in implements {
                        match mapping(&ori_impl) {
                            Some(mapped_impl) => mapped_impls.push(mapped_impl),
                            None => mapped_impls.push(ori_impl.clone())
                        }
                    }

                    Some(Elements::Class(ancestors.clone(), annotations.clone(), modifiers.clone(), name.to_string(), mapped_extends, mapped_impls))
                },
                // ancestors, annotations, modifiers, name, extends
                Elements::Interface(ancestors, annotations, modifiers, name, extends) => {
                    let mut mapped_extends: Vec<TypeDescriptor> = Vec::new();
                    for ori_extend in extends {
                        match mapping(ori_extend) {
                            Some(mapped_extend) => mapped_extends.push(mapped_extend),
                            None => mapped_extends.push(ori_extend.clone())
                        }
                    }

                    Some(Elements::Interface(ancestors.clone(), annotations.clone(), modifiers.clone(), name.to_string(), mapped_extends))
                },
                // ancestors, modifiers, field type, field name, field value
                Elements::Field(ancestors, modifiers, field_type, field_name, field_value) => {
                    let mut mapped_field_type = None;
                    if let Some(td) = field_type {
                        mapped_field_type = mapping(td);
                        if mapped_field_type.is_none() {
                            mapped_field_type = field_type.clone();
                        };
                    }

                    Some(Elements::Field(ancestors.clone(), modifiers.clone(), mapped_field_type, field_name.clone(), field_value.clone()))
                },
                // ancestors, annotation, modifiers, return type, function name, params(variable(modifier, type, name))
                Elements::Method(ancestors, annotations, modifiers, ret_type, func_name, params) => {
                    let mapped_ret_type = match mapping(ret_type) {
                        Some(mapped_rt) => mapped_rt,
                        None => ret_type.clone()
                    };

                    let mut mapped_params: Vec<ParamDescriptor> = Vec::new();
                    for param in params {
                        let mapped_param_type = match mapping(&param.ty) {
                            Some(ty) => ty,
                            None => param.ty.clone()
                        };
                        mapped_params.push(ParamDescriptor { modifiers: param.modifiers.clone(), ty: mapped_param_type, name: param.name.clone() });
                    }

                    Some(Elements::Method(ancestors.clone(), annotations.clone(), modifiers.clone(), mapped_ret_type, func_name.clone(), mapped_params))
                },
                // ancestors, modifiers, ident, params(modifiers, type, name)
                Elements::Constructor(ancestors, modifiers, ident, params) => {
                    let mut mapped_params: Vec<ParamDescriptor> = Vec::new();
                    for param in params {
                        let mapped_param_type = match mapping(&param.ty) {
                            Some(ty) => ty,
                            None => param.ty.clone()
                        };
                        mapped_params.push(ParamDescriptor { modifiers: param.modifiers.clone(), ty: mapped_param_type, name: param.name.clone() });
                    }

                    Some(Elements::Constructor(ancestors.clone(), modifiers.clone(), ident.clone(), mapped_params))
                },
                // package, name, rest 
                // TODO:
                Elements::CreatorCall(creator_type, rests) => None,
                // cast, caller, method name, params((annotation, type, name))
                Elements::MethodCall(cast, caller, method_name, params) => {
                    let mut mapped_caller = match mapping(&caller.ty) {
                        Some(mapped_caller_type) => Caller {
                            ty: mapped_caller_type,
                            name: caller.name.clone(),
                        },
                        None => caller.clone()
                    };
                    
                    Some(Elements::MethodCall(cast.clone(), mapped_caller, method_name.clone(), params.clone()))
                },
                _ => None
            } {
                vertex.set_type(Some(new_vt));
            }
        }
    }
}

pub struct JavaMapperPolicy;

impl JavaMapperPolicy {
    pub fn load_vertex_from_file(&self, path: &PathBuf) -> LanguageAnalysisResult<HashMap<ElementCategories, Vec<JavaElement>>> {
        let context_str = fs::read_to_string(path).expect("should read context of file");
        
        let mut deserializer = serde_json::Deserializer::from_str(&context_str);
        deserializer.disable_recursion_limit();
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

        match Value::deserialize(deserializer) {
            Ok(v) => match serde_json::from_value(v) {
                Ok(contents) => Ok(contents),
                Err(e) => Err(LanguageAnalysisPolicyError {})
            },
            Err(e) => Err(LanguageAnalysisPolicyError {})
        }
    }

    fn collecting(&self, data: &HashMap<ElementCategories, Vec<JavaElement>>, listener: &mut JavaDataMappingListener) {
        let get_combined_name = |names: &Vec<String>, name: &String| -> Vec<String> {
            let mut combined_names = names.clone();
            combined_names.push(name.to_string());
            combined_names
        };

        // collecting all types
        for (cate, jvs) in data {
            match cate {
                ElementCategories::Classes | ElementCategories::Interfaces => {
                    for jv in jvs {
                        if let Some(vt) = jv.get_type() {
                            if listener.types.contains_key(&vt.to_string()) {
                                continue;
                            }

                            if let Some(td) = match vt {
                                // VertexType::Import(ty) => Some(ty.clone()),
                                // ancestors, annotations, modifiers, name, extends, implements
                                Elements::Class(ancestors, _, _, name, _, _) =>
                                    Some(TypeDescriptor { package: ancestors.package.clone(), name: get_combined_name(&ancestors.name, name) }),
                                // ancestors, annotations, modifiers, name, extends
                                Elements::Interface(ancestors, _, _, name, _) =>
                                    Some(TypeDescriptor { package: ancestors.package.clone(), name: get_combined_name(&ancestors.name, name) }),
                                _ => None

                            } {
                                listener.types.insert(td.to_string(), td);
                            }
                        }
                    }
                },
                _ => ()
            }
        }
    }

    fn mapping(&self, data: &mut HashMap<ElementCategories, Vec<JavaElement>>, listener: &mut JavaDataMappingListener) {
        // go through the vertexes
        for (_, jvs) in data {
            for jv in jvs {
                self.vertex_tree_walker(jv, listener)
            }
        }
    }

    fn vertex_tree_walker(&self, vertex: &mut JavaElement, listener: &mut JavaDataMappingListener) {
        listener.map(vertex);

        for cate in ElementCategories::iter() {
            if let Some(members) = vertex.get_member_by_category_mut(cate) {
                for jv in members {
                    listener.map(jv);
                }
            }
        }
    }
}

impl LanguageMapPolicy for JavaMapperPolicy {
    fn new() -> Self {
        JavaMapperPolicy {}
    }

    fn execute(&mut self, data: &PathBuf, output: &PathBuf) -> LanguageMapResult<PathBuf> {
        // collect all files
        let mut filelist: Vec<PathBuf> = Vec::new();
        if data.is_file() {
            filelist.push(data.to_path_buf());
        } else {
            let filename_pattern = format!("{}/**/*.{}", data.to_str().unwrap(), ELEMENT_FILE_EXTENSION);
            if let Some(mut files) = utils::list_files(data, filename_pattern.as_str(), &Vec::new()) {
                filelist.append(&mut files);
            }
        }

        let mut vertex_listener = JavaDataMappingListener {
            types: HashMap::new()
        };

        // load all vertexes
        println!("# Loading files ...");
        let total = filelist.len();
        let mut index = 1;
        let mut all_vertexes: HashMap<ElementCategories, Vec<JavaElement>> = HashMap::new();
        for vertex_file in filelist {
            print!(" - [{} / {}] loading {} ... ", index, total, vertex_file.to_str().unwrap());

            if let Ok(vertexes) = self.load_vertex_from_file(&PathBuf::from(vertex_file)) {
                for (cate, mut vertex_list) in vertexes {
                    if cate == ElementCategories::Package || cate == ElementCategories::Imports {
                        continue;
                    }

                    match all_vertexes.get_mut(&cate) {
                        Some(list) => list.append(&mut vertex_list),
                        None => _ = all_vertexes.insert(cate, Vec::from(vertex_list))
                    };
                }
            }

            println!("done");
            index += 1;
        }
        println!("-------------------------------------------------");
        println!(" {} files loaded\n", total);

        // collect all types
        print!("# collecting types ... ");
        self.collecting(&all_vertexes, &mut vertex_listener);
        println!("done");
        println!("-------------------------------------------------");
        println!(" total: {} types collected\n", vertex_listener.types.len());

        // mapping types
        print!("# mapping types ... ");
        self.mapping(&mut all_vertexes, &mut vertex_listener);
        println!("done\n");

        // save all mapped vertexes
        let mapped_vertexes_file = output.join(format!("{}.{}", "all", ELEMENT_FILE_EXTENSION));
        if let Ok(mut f) = File::create(&mapped_vertexes_file) {
            let _ = f.write_all(serde_json::to_string(&all_vertexes).unwrap().as_bytes());
            let _ = f.flush();
        }
        Ok(mapped_vertexes_file)
    }
}