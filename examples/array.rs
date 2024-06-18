#[derive(Copy, Clone, Debug, typeful::EnumFunctions)]
#[enum_functions(variant_array, variant_count)]
enum TestEnum {
    VariantA,
    VariantB,
    VariantC,
}

impl TestEnum {
    const VARIANT_ARRAY: [TestEnum; 3] =
        Self::variant_array::<{ Self::VARIANT_COUNT }>();
    const VARIANT_COUNT: usize = Self::variant_count();
}

fn main() {
    println!("{}; {:?}", TestEnum::VARIANT_COUNT, TestEnum::VARIANT_ARRAY);
}
