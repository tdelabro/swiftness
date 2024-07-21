use cairovm_verifier_air::layout::recursive::global_values::InteractionElements;
use cairovm_verifier_commitment::{
    table::{config::Config as TableCommitmentConfig, types::Commitment as TableCommitment},
    vector::{config::Config as VectorCommitmentConfig, types::Commitment as VectorCommitment},
};
use starknet_crypto::Felt;

use crate::types::StarkCommitment;

use super::oods_values;

pub fn get() -> StarkCommitment<InteractionElements> {
    StarkCommitment {
        traces: cairovm_verifier_air::fixtures::commitment::get(),
        composition: TableCommitment {
            config: TableCommitmentConfig {
                n_columns: Felt::from_hex_unchecked("0x2"),
                vector: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
            },
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: Felt::from_hex_unchecked(
                    "0x30b93bbd6b193eb57d9f818202b899b7e8e09b0c7d183537fe85f4e6b6f4373",
                ),
            },
        },
        interaction_after_composition: Felt::from_hex_unchecked(
            "0x1c0d29e24bc79e4679b20a2185841d65fa77a288bdd472e1d4a3de9324338e7",
        ),
        oods_values: oods_values::get(),
        interaction_after_oods: vec![
            Felt::from_hex_unchecked("0x1"),
            Felt::from_hex_unchecked(
                "0x7868c5d7840087a855b30079a12cc1d5eeaedb666607b31e33175f41d3b535f",
            ),
            Felt::from_hex_unchecked(
                "0x28dbb2cfd00dae2334af761e1bd021bf8021bf2c30d3c75e075d6741ce836a0",
            ),
            Felt::from_hex_unchecked(
                "0xa0a779b5f29842287ca6ad3dffd9018d9a2a37007e18ae898ca4526a9a2691",
            ),
            Felt::from_hex_unchecked(
                "0x53b0e6a89c16e607a26ad5825ff22f14b1b37a7a4f8ee6cce668d15a3648517",
            ),
            Felt::from_hex_unchecked(
                "0x4a56b7d078479bfd71ec6e1c8a374849642f9be4b177a4c71ccd071f87a9001",
            ),
            Felt::from_hex_unchecked(
                "0x24c24c299414aefb657b7eb37f51b3c52bffd6b316bce5d50a594521045a3f4",
            ),
            Felt::from_hex_unchecked(
                "0x71bacb4a16d6dfb5748d1a8e584ae5581d6e33b1abe4f6a491308868b70922c",
            ),
            Felt::from_hex_unchecked(
                "0x27883bc4bd5b8d46948d87334b08d7270601fdb2917c87102537c8d855e17e",
            ),
            Felt::from_hex_unchecked(
                "0x31675df9ac822dc19b536e6e4004c7ae6c9d1e260072c77a8de26eedfc54892",
            ),
            Felt::from_hex_unchecked(
                "0x7ce2971d07b3bfb37262d1d0c27cbbb7cad88db92b4c4bd27b808a379152f9b",
            ),
            Felt::from_hex_unchecked(
                "0x48f786362233207cc8f6005a586dadb865ea42060dcf3cf5fafd463524f4451",
            ),
            Felt::from_hex_unchecked(
                "0xea197a93ef678fa1baf4eb89605f8909a9c0077e7b59fbe3b5e98ef551de0f",
            ),
            Felt::from_hex_unchecked(
                "0x7a0f8520c76d89458269bd95ca9a5256d8b8de284c7404439a59a1d84f05eae",
            ),
            Felt::from_hex_unchecked(
                "0x3d4e323c595da4f57d0f522d8a8ab290fc748c528907ff3c6ab36508ae08461",
            ),
            Felt::from_hex_unchecked(
                "0x50a7aaf380a5b11b7e7f27c368a8bab68af9a1e284ff1f8077f8a93ec9eba89",
            ),
            Felt::from_hex_unchecked(
                "0x672abc6b56e6c8e86f6f1713cf84800b584e5bb9868fd214e03aa82eb761147",
            ),
            Felt::from_hex_unchecked(
                "0x525887e108a04f6670f02a5eebd267c39616beadde7629879bd417e26440076",
            ),
            Felt::from_hex_unchecked(
                "0x566ac532992aa6102365c9f532ac5580dd10d6ee417288edc100425d7306b71",
            ),
            Felt::from_hex_unchecked(
                "0x1a186fc1b4b5ee8431454f616b04c2ab2c2b2e6454150808af7f3880e35f79a",
            ),
            Felt::from_hex_unchecked(
                "0x73a62b0a99dc3678f3829fe586e145f7776975e693e69768d6a55d4c8f36fee",
            ),
            Felt::from_hex_unchecked(
                "0xa400aad225ee0fbd64537626b2612d1cc944708d59f006fef9be305a30d295",
            ),
            Felt::from_hex_unchecked(
                "0x5779b9e99e1a0f7e52d9dac2505af4bbb06a009944f35e5dd5f781631d06d1c",
            ),
            Felt::from_hex_unchecked(
                "0x7ad6019ffc515cd5ab68bb68873a1cdf3d6de72a2db6dc764c98454b2f69c63",
            ),
            Felt::from_hex_unchecked(
                "0x85d80b3054fd0b87bd20ed0f8c2f29f38fad604e1ee95d0445c77b5e99ab91",
            ),
            Felt::from_hex_unchecked(
                "0x30c4ee130c47e053771660990dcb5223a713e95f687e177fb4b4c66d7bffa92",
            ),
            Felt::from_hex_unchecked(
                "0x1366ff50f78d3cbb946503fe9f31dd772ef42c4c192a8847e1111e8eb23a7da",
            ),
            Felt::from_hex_unchecked(
                "0x50dc3456f742c0d9268d2875cab9269600b0b6d031b114e3afa853f8726dac3",
            ),
            Felt::from_hex_unchecked(
                "0x375376fd8a86df55c28b32e17e4f1ebf7a89f80f803c7ee28928172a9f467db",
            ),
            Felt::from_hex_unchecked(
                "0xa493f31f208795fc2b62b2da8d0f6844543a1b462f3ed4b2c83a5d2bf02482",
            ),
            Felt::from_hex_unchecked(
                "0x2a084ad1f02a578edc944f5332363d3f36f01d593a41b1c780c489b89a489d4",
            ),
            Felt::from_hex_unchecked(
                "0x53d6cf238f4dd2bc47147993b629c04d99361b8d324428c49b783bd684fa643",
            ),
            Felt::from_hex_unchecked(
                "0x311beec9a7ace3c500834b2562d048a52d4a0035da10a499e0a705993b70484",
            ),
            Felt::from_hex_unchecked(
                "0x24983c1a9b51e98b9fa4a5c1951eef142ac8cfc1a5b00a1d832146b1be155b0",
            ),
            Felt::from_hex_unchecked(
                "0x4a60957c67cc4f897cf25ad58f46524d8c137b784572adb5737e679e81e9492",
            ),
            Felt::from_hex_unchecked(
                "0x2a67e5b49f7ea96e60b1b6f848ae4a5b7b77256718a9f240474e421ec475a82",
            ),
            Felt::from_hex_unchecked(
                "0xd3d3e58527bc4aa8c8516e0c75b16b7d383d61766bf42d405adfb2868cecb3",
            ),
            Felt::from_hex_unchecked(
                "0x49eb5bfdb52b10343b535a8da671e5537f76dbb082ca80d327a7d1ced1c1813",
            ),
            Felt::from_hex_unchecked(
                "0x5dcff63507da18efc8f32d520061a36104aeb2249d537c96c43a7c7143ffd7a",
            ),
            Felt::from_hex_unchecked(
                "0x43ef0837ba12f3a7954f82a054f2d9a81046ac989bb062b25e192e3da88657a",
            ),
            Felt::from_hex_unchecked(
                "0x19a09684b294b83a9e0fecdb1584f96d1ca69ee8cab3a712cdfcf9fffd8a009",
            ),
            Felt::from_hex_unchecked(
                "0x390c0a24e2d8e1a01326199fc6008e0e7b463d9852d57d110bdc59c42f17d0a",
            ),
            Felt::from_hex_unchecked(
                "0x504d2603cdfc2b30132e63d008313f2167860895df68fcbfa9d06e14bf8449a",
            ),
            Felt::from_hex_unchecked(
                "0x3b7e6118d1d8464495a7da4fd97246675c4df11b018523d52f1d74244ea7c0c",
            ),
            Felt::from_hex_unchecked(
                "0x1d82df8727f2f6bc595814269646919aa7abef2a036a37d43b8d8e39fc29ea4",
            ),
            Felt::from_hex_unchecked(
                "0x738f7a9d17b7b662741d7f8483e48e92ad09143029a6714812b6191e2238f7e",
            ),
            Felt::from_hex_unchecked(
                "0x4c11c2f470ae3ece6195dff93b6a0bc57977ecb2b43e96acc5f1ac48e29ffd3",
            ),
            Felt::from_hex_unchecked(
                "0x21f328e55205e9258ca14096d7bf81b210b60c7c101046085264aad9611b135",
            ),
            Felt::from_hex_unchecked(
                "0x485052252bcc49e1ea8acc424f6108827496ee7cc8f5683de2cf0478d184f9d",
            ),
            Felt::from_hex_unchecked(
                "0x63c3ed7402083ce2c5b87abd46cbdbe322f22ff8ce237b91c077cf2c8d37b2a",
            ),
            Felt::from_hex_unchecked(
                "0x32c3f7b8c5427b30755e4f85bbb54d898d67fc8d3b4614c819819753dcc0c88",
            ),
            Felt::from_hex_unchecked(
                "0x6bb6f22717be61fa33748bfac49d676d7cb3a377c1e069bf4084c284a57998f",
            ),
            Felt::from_hex_unchecked(
                "0x1d4ee2c5711daa03b9639ef283fdd239f5c2bad67725f3cde07aced2599807d",
            ),
            Felt::from_hex_unchecked(
                "0x2900acfeb87a24ab2a68439f8c1ebd9cbbb18f7098b7005c903ea0fac3b57d3",
            ),
            Felt::from_hex_unchecked(
                "0x1d3dc927a04676a00b13136d8b0f4e4c727f32eca4d0fa65a5c37a805ff5fbe",
            ),
            Felt::from_hex_unchecked(
                "0x64e7c25c902ce83ade21bec644d8a900e28a1513133f93c527e11843c605aa",
            ),
            Felt::from_hex_unchecked(
                "0x687d799979e8d19a80487df5a25718162f84ac76df75c13cee8f618fcad89b5",
            ),
            Felt::from_hex_unchecked(
                "0xb250457f66f4a78ba2c5017c23f601e90e0760cdb33e24c02860d1e13df3c4",
            ),
            Felt::from_hex_unchecked(
                "0x3a13b1ad56d28758f738fd802a0480ef904288870178fc02d7a4703f451a287",
            ),
            Felt::from_hex_unchecked(
                "0x700824bff100e6ee21146b5e87a7b75263fd95ef9b874ce510c3f4343e78b96",
            ),
            Felt::from_hex_unchecked(
                "0x529d39dc2c40d2361dd6bb58a1ad002cc6a61aa1a1d3a94b23e8e31aa53f437",
            ),
            Felt::from_hex_unchecked(
                "0x6e2b918db039cc6c8babc5277147573471c949b9e953d2adc75dcf8611262e2",
            ),
            Felt::from_hex_unchecked(
                "0x2f12ad7fffc00e63f6e4f64c711b95113ed1938de094eb547e6cc49c2321c7e",
            ),
            Felt::from_hex_unchecked(
                "0x6518b0f525c8734a23ca5b18a956d3e905e9fe4d973ced1d57e8e625e61a3c5",
            ),
            Felt::from_hex_unchecked(
                "0x212b9c1993150df51af590e21475fab5047f1eea0d287ec5c0dcc3417c65124",
            ),
            Felt::from_hex_unchecked(
                "0x66a7273f0082132160a243e68b5993c7ec54b93a6cf65eee94e799de4fa53cd",
            ),
            Felt::from_hex_unchecked(
                "0x31354d753bcf44b5095b426450313102e3f93f066e690ed9989c0819285a3a5",
            ),
            Felt::from_hex_unchecked(
                "0x1d7e1f9410bc2f92921a49868e53cabdeaa790c00b3faa5d143eecdff3edd10",
            ),
            Felt::from_hex_unchecked(
                "0x60a545f6841027952bb2727adfca1d1563485a8bad43c00da8fbc0210eff360",
            ),
            Felt::from_hex_unchecked(
                "0x101d4ae866b4d9dde4f0044540c4db000368f085c8519aeb9de6c6f2fb5a7aa",
            ),
            Felt::from_hex_unchecked(
                "0x2105965bf04b6634f473f2e90650c0a3faf8c189aabe90105ebd7cc484557c6",
            ),
            Felt::from_hex_unchecked(
                "0x7a1ba866d82ba575bb06bfe4c0a4a178d20213100692ee4e95fe7ef71e3e1a",
            ),
            Felt::from_hex_unchecked(
                "0x215dec25e592192a205946e4fb13448f65b5d42c2219c550233c29c1b28939a",
            ),
            Felt::from_hex_unchecked(
                "0x687fcc8e56012bf95e5016ff81357015ec2f7fb28ac0bbdc9b0c8f4a380590d",
            ),
            Felt::from_hex_unchecked(
                "0x233cbd274d8b58460d0264b1cc6ebb49d1fd4d028ffa8090ca31615d30e79f3",
            ),
            Felt::from_hex_unchecked(
                "0x691ad7a23a5ea05d508ad01202e6b43812a6a334eb4f77b64b756c386f051e2",
            ),
            Felt::from_hex_unchecked(
                "0x2c69c4c3dcd8775145002146f651da0543a32dd17c4ab4a0b36adacd2a34ab7",
            ),
            Felt::from_hex_unchecked(
                "0x549a7522f40646a59a057a707d54507eb0ed13f7e8d177df77d3f272f548a06",
            ),
            Felt::from_hex_unchecked(
                "0x30b02a6d1e321215531cbf98208609549a15b716e257ab61832e5848ffa165a",
            ),
            Felt::from_hex_unchecked(
                "0x4873056097b5936ac52bc208c2d7aa42e4b19ce0b37fde0d79cd5391fc3379b",
            ),
            Felt::from_hex_unchecked(
                "0x644159b987e670f37296a8ad9e29d353919c76b7aa342e202550bc2bac9f1a2",
            ),
            Felt::from_hex_unchecked(
                "0x6ae845044e4a9637400b4ea0f0b2989eb1c1357a09ef4dca5e1734742fd6a4a",
            ),
            Felt::from_hex_unchecked(
                "0x15645602c7c6c433c962ec235abec69d013d13b695e0f092e2c58cd4419ca19",
            ),
            Felt::from_hex_unchecked(
                "0x95a9bbfb14d9d5b1c5e76c6ff8e3e70ad0a673af9b9627b08f8219ad45cec8",
            ),
            Felt::from_hex_unchecked(
                "0x36102f1b66758ddfdea4b1972b5ac2f92ecc6106d965c8d0b57dcdd1b97154d",
            ),
            Felt::from_hex_unchecked(
                "0x4fc697198a5f2668d86ffaa95c126f8ba0d7df780476501eaa54ce0dd94e238",
            ),
            Felt::from_hex_unchecked(
                "0x10c5ca87a4a8cbd70e8d06a010a732b04dc732a896e6e4c6a01b043c64f9c79",
            ),
            Felt::from_hex_unchecked(
                "0x24d88b691a9ea915f9cff213adc7809386417ad8a543ec20adf245b7832680d",
            ),
            Felt::from_hex_unchecked(
                "0x26da6927270ccc76639f7674441c68db2643b9fa93345ed510f258e326b0d79",
            ),
            Felt::from_hex_unchecked(
                "0x3ed1745fd1b8662af5f691b0d84c8ebd9d2f029b4583c7f1ff2b3d2f010e7e1",
            ),
            Felt::from_hex_unchecked(
                "0x32472b1ebfaccf2eaf232d17b8152aef1a1ef35666ec627215f66817a5ca4f7",
            ),
            Felt::from_hex_unchecked(
                "0x111c03561bc1c7d025c511e451815f8a2b2f5850a70a4faa46ebe28a64ba626",
            ),
            Felt::from_hex_unchecked(
                "0x7c3119123eaaadf62974c0333bb4ed999d37891987e66a5b4444ccc2b88651a",
            ),
            Felt::from_hex_unchecked(
                "0x6cfd7778882ec7145f6e999e0d1c177cc66542213f9ff710b91d81d0a84bead",
            ),
            Felt::from_hex_unchecked(
                "0x3217c9ac6e0e7d6dd6f3a9f6737ed90fb91be37c70c2095e0bba34676726e80",
            ),
            Felt::from_hex_unchecked(
                "0x4911dcc079e3fce97ff17cb85f22480a490cba7e7b83f628e67620bbf93e86d",
            ),
            Felt::from_hex_unchecked(
                "0x541e206771de8454d88946900166ad1498f1c44d75287bd6e664f8e91b3fd0c",
            ),
            Felt::from_hex_unchecked(
                "0x5a831acd914777a71daef7a3e732bd421eaaddc87fbcea8d000cc5c2fad27e2",
            ),
            Felt::from_hex_unchecked(
                "0x571982ce59262f88940de7c4eb3ade0f640fb5cace6f944460c496db257772b",
            ),
            Felt::from_hex_unchecked(
                "0x326cdb5e3e572ea92c3534ea38d25f8eb67fa3aeebf8203a26db6cc9866856f",
            ),
            Felt::from_hex_unchecked(
                "0x18a478a5e8f7eac8302a011b946256f9419e773af2143fc5d35e2888b3c0dc1",
            ),
            Felt::from_hex_unchecked(
                "0x72c03ebf725206ee3e8da302d20b3a37f2d3ed459d2558f2e6d8116b80d1785",
            ),
            Felt::from_hex_unchecked(
                "0x4dcaaf578b506bd84cff928af05dcc6b2620546d172e43cceffb67b5a206c1e",
            ),
            Felt::from_hex_unchecked(
                "0x1f37145d32a5066194ab8b71ebdfc0a0dd1adf9fe3ce3259d7caa013061ab85",
            ),
            Felt::from_hex_unchecked(
                "0x6c83957c138ae404609390956a40c0da37f8727d34306ad9c4dcd1f7cfad7de",
            ),
            Felt::from_hex_unchecked(
                "0x4b9e66c32d6b90c49f691ddc606c060a52a14e2bebc3705ab7304079ff55ce7",
            ),
            Felt::from_hex_unchecked(
                "0x56dada58b5a8bc3eb0e68fcbe3ce8e64f397829b050302b8a9058806774c815",
            ),
            Felt::from_hex_unchecked(
                "0x4589baff9a4e06756fca4d5532bafb56bdc8e72c9f41c5cdf969153d066123e",
            ),
            Felt::from_hex_unchecked(
                "0xcee152497866b1b0d5bb9341781f42c8d9777c94188851cae94868c0078491",
            ),
            Felt::from_hex_unchecked(
                "0x4502dc84f2f3786360d7794a1c7cfda27d49184ec17b2a2341514200da3dad4",
            ),
            Felt::from_hex_unchecked(
                "0x35c6954882d08ab1a6dd181def18c9764000b9f9831146cc39906b6da9466dd",
            ),
            Felt::from_hex_unchecked(
                "0x3f2283130bb26ccb827407312d1f9dabff4d836d4d12c16fd7076c0d828a4b3",
            ),
            Felt::from_hex_unchecked(
                "0x6fca8e2aa9223cba3ee1eb02505612b7819d12ffc5c71456408c8ff68e3c08a",
            ),
            Felt::from_hex_unchecked(
                "0x2b989709f8d7f0ade477f131580340a946934c919e2034d3992c12d4b620ad6",
            ),
            Felt::from_hex_unchecked(
                "0x302984172abe1cc0fd9b56037adaa17121a1c25715d54150e9b9591ad5c05a6",
            ),
            Felt::from_hex_unchecked(
                "0x3ccf277837817cfa1874bd425c76a2f630c0a53fdad8148febb0d4328b7323e",
            ),
            Felt::from_hex_unchecked(
                "0x5cf3ad2f296934328999b16c1e9df52553c54d0262953fff91f97269e5ccfb3",
            ),
            Felt::from_hex_unchecked(
                "0x62016b10caf536b5bf30653c3f70ca59c414d923dd853bc3350f2a0e3926da",
            ),
            Felt::from_hex_unchecked(
                "0xa979e1d89fb72dd78e08493bc35fc01de84b8c817e701286bc52fc406f2975",
            ),
            Felt::from_hex_unchecked(
                "0x691872b561270cf8d34259464eae1e3eaee74aa97abf5d2dd6e82387e5cd58c",
            ),
            Felt::from_hex_unchecked(
                "0xbb9cfaa2114657f8d4a9a4394caec24ba24526bb18997f18b8ec960ec66110",
            ),
            Felt::from_hex_unchecked(
                "0x42923db6350cc1e28787e770f5ef18632378638452d64ddcddc4b0754145bc1",
            ),
            Felt::from_hex_unchecked(
                "0x5c7f5aa242c39d158d3607f2c06c35776ddf61baf29b2cc01ff7c1f0f8da753",
            ),
            Felt::from_hex_unchecked(
                "0x2f5c581fb811c24ae5ee543ddbc950712f7e07b6c9105deb9cafcd4ad2dce6b",
            ),
            Felt::from_hex_unchecked(
                "0x5762b8182694a222b32a80f95967c6d5bb3c8058857fd263311eb9b1974e37a",
            ),
            Felt::from_hex_unchecked(
                "0x2413e7815b1965304a941a4cfdfa6d5a3d5421b7604744bbf307359a9118354",
            ),
            Felt::from_hex_unchecked(
                "0x37e7b12db964e3387ca2ced152dad129e4440fa5d40e0a869c1dc139cb89039",
            ),
            Felt::from_hex_unchecked(
                "0x3f8666925feadaea42c59ca59000f215cde3f3822a1c0b5535f0c4cfd218d5b",
            ),
            Felt::from_hex_unchecked(
                "0x1b3b5d5b5d037e1c08818f9b21a521cc355b76ddf3aa9deb40e8027567ec529",
            ),
            Felt::from_hex_unchecked(
                "0x74428bd541585814c94762ade8454e0c868ccac6264b56ec42144b32b0350bf",
            ),
            Felt::from_hex_unchecked(
                "0x6c2bfe5c87fa4817b47dfea851cc65a63eb208eba3ef8d8ff9cc4d5ccc38162",
            ),
            Felt::from_hex_unchecked(
                "0x6dcf7f9d643754dde755f59b7e99d23fa8fe3873081d61cef76746e42f2c2bf",
            ),
            Felt::from_hex_unchecked(
                "0x2fb76d734073db3e5f46085ec5058f02ccab70d70c31508c22e659ed00337ae",
            ),
            Felt::from_hex_unchecked(
                "0x75bcd848a595eeed1e03977dd35f052ebf6a15306961e4fc57670916663e15e",
            ),
            Felt::from_hex_unchecked(
                "0x7a702a902f300a47fb79f18195691f2f1cd57fa870b6d4e12d365ed59231506",
            ),
        ],
        fri: cairovm_verifier_fri::fixtures::commitment::get(),
    }
}
