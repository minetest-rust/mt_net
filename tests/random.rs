use libtest_mimic::{Arguments, Trial};

use mt_net::{generate_random::GenerateRandomVariant, rand, ToCltPkt, ToSrvPkt};
use mt_ser::{DefCfg, MtDeserialize, MtSerialize};
use std::{error::Error, fmt::Debug};

fn test_reserialize<T>(type_name: &'static str) -> impl Iterator<Item = Trial>
where
    T: MtSerialize + MtDeserialize + GenerateRandomVariant + PartialEq + Debug,
{
    (0..T::num_variants()).map(move |i| {
        Trial::test(format!("{type_name}::{}", T::variant_name(i)), move || {
            let mut rng = rand::thread_rng();

            for _ in 0..100 {
                let input = T::generate_random_variant(&mut rng, i);

                let mut writer = Vec::new();
                input
                    .mt_serialize::<DefCfg>(&mut writer)
                    .map_err(|e| format!("serialize error: {e}\ninput: {input:?}"))?;

                let mut reader = std::io::Cursor::new(writer);
                let output = T::mt_deserialize::<DefCfg>(&mut reader).map_err(|e| {
                    format!(
                        "deserialize error: {e}\ninput: {input:?}\npayload: {:?}",
                        reader.get_ref()
                    )
                })?;

                if input != output {
                    return Err(format!(
                        "output did not match input\n\
						input: {input:?}\n\
						payload: {:?}\n\
						output: {output:?}",
                        reader.get_ref(),
                    )
                    .into());
                }
            }

            Ok(())
        })
        .with_kind("random")
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::from_args();
    let tests = test_reserialize::<ToSrvPkt>("ToSrvPkt")
        .chain(test_reserialize::<ToCltPkt>("ToCltPkt"))
        .collect();
    libtest_mimic::run(&args, tests).exit();
}
