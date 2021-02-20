use vergen::ConstantsFlags;

fn main() -> Result<(), vergen::Error> {
    vergen::gen(
        ConstantsFlags::BUILD_DATE
            | ConstantsFlags::SHA_SHORT
            | ConstantsFlags::REBUILD_ON_HEAD_CHANGE,
    )
}
