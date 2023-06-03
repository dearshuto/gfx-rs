use core::fmt;

pub struct Hlsl;
pub struct Glsl;
pub struct SpirV;
pub struct Wgsl;

pub struct ShaderConverter<TDst, TSrc> {
    _marker: std::marker::PhantomData<(TDst, TSrc)>,
}

impl ShaderConverter<Hlsl, Glsl> {
    pub fn convert_glsl_to_hlsl(source: &str) {
        let stage = naga::ShaderStage::Compute;
        let options = naga::front::glsl::Options::from(stage);
        let module = naga::front::glsl::Frontend::default()
            .parse(&options, source)
            .unwrap();

        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();

        let options = naga::back::hlsl::Options::default();
        let mut data = naga::back::hlsl::Writer::new(A {}, &options);
        data.write(&module, &info).unwrap();
    }
}

impl ShaderConverter<Wgsl, Glsl> {
    pub fn convert_glsl_to_wgsl(source: &str) -> String {
        let stage = naga::ShaderStage::Compute;
        let options = naga::front::glsl::Options::from(stage);
        let module = naga::front::glsl::Frontend::default()
            .parse(&options, source)
            .unwrap();

        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();

        let flags = naga::back::wgsl::WriterFlags::all();
        naga::back::wgsl::write_string(&module, &info, flags).unwrap()
    }
}

impl ShaderConverter<SpirV, Wgsl> {
    pub fn convert_wgsl_to_spirv(source: &str) -> Vec<u8> {
        let module = naga::front::wgsl::parse_str(source).unwrap();
        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();

        let options = naga::back::spv::Options::default();
        let mut data = naga::back::spv::write_vec(&module, &info, &options, None).unwrap();

        let ratio = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();
        let length = data.len() * ratio;
        let capacity = data.capacity() * ratio;
        let ptr = data.as_mut_ptr() as *mut u8;
        unsafe {
            let u8_data: Vec<u8> = Vec::from_raw_parts(ptr, length, capacity).clone();

            // 元データが 2 重に破棄されないように、元データを破棄しないようにする
            std::mem::forget(data);

            u8_data
        }
    }
}

// GLSL -> HLSL の変換結果を出力するために必要なやつ
struct A;
impl fmt::Write for A {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print!("{}", s);
        fmt::Result::Ok(())
    }
}
