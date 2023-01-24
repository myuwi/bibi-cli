pub struct Channel {
    pub id: &'static str,
    pub handle: Option<&'static str>,
    pub name: &'static str,
    pub branch: Branch,
}

pub enum Branch {
    Hololive,
    Holostars,
}

pub const CHANNELS: &[Channel] = &[
    Channel {
        id: "UCJFZiqLMntJufDCHc6bQixg",
        handle: Some("@hololive"),
        name: "hololive ホロライブ - VTuber Group",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCWsfcksUUpoEvhia0_ut0bA",
        handle: Some("@HOLOSTARS"),
        name: "holostars ホロスターズ - VTuber Group",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCfrWoRGlawPQDQxxeIDRP0Q",
        handle: Some("@hololiveIndonesia"),
        name: "hololive Indonesia",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCotXwY6s8pWmuWd_snKYjhg",
        handle: Some("@hololiveEnglish"),
        name: "hololive English",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCJxZpzx4wHzEYD-eCiZPikg",
        handle: Some("@HOLOSTARSenglish"),
        name: "HOLOSTARS English",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCp6993wxpyDPHUpavwDFqgg",
        handle: Some("@TokinoSora"),
        name: "SoraCh. ときのそらチャンネル",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCDqI2jOz0weumE8s7paEk6g",
        handle: Some("@Robocosan"),
        name: "Roboco Ch. - ロボ子",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC5CwaMl1eIgY8h02uZw7u8A",
        handle: Some("@HoshimachiSuisei"),
        name: "Suisei Channel",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC-hM6YJuNYVAmUWxeIr9FeA",
        handle: Some("@SakuraMiko"),
        name: "Miko Ch. さくらみこ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC0TXe_LYZ4scaW2XMyi5_kw",
        handle: Some("@AZKi"),
        name: "AZKi Channel",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC1CfXB_kRs3C-zaeTG3oGyg",
        handle: Some("@AkaiHaato"),
        name: "HAACHAMA Ch 赤井はあと",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCD8HOxPs4Xvsm8H0ZxXGiBw",
        handle: Some("@YozoraMel"),
        name: "Mel Channel 夜空メルチャンネル",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCdn5BQ06XqgXoAxIhbqw5Rg",
        handle: Some("@ShirakamiFubuki"),
        name: "フブキCh。白上フブキ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCFTLzh12_nrtzqBPsTCqenA",
        handle: Some("@AkiRosenthal"),
        name: "アキロゼCh。Vtuber/ホロライブ所属",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCQ0UDLQCjY0rmuxCDE38FGg",
        handle: Some("@NatsuiroMatsuri"),
        name: "Matsuri Channel 夏色まつり",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCHj_mh57PVMXhAUDphUQDFA",
        handle: Some("@AkaiHaato_Sub"),
        name: "HAACHAMA DARKWEB CH",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCLbtM3JZfRTg8v2KGag-RMw",
        handle: Some("@user-yp5xk6qh4q"),
        name: "AkiRose Ch.アキ・ローゼンタールSub",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC1opHUrw8rvnsadT-iGp7Cg",
        handle: Some("@MinatoAqua"),
        name: "Aqua Ch. 湊あくあ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC1suqwovbL1kzsoaZgFZLKg",
        handle: Some("@YuzukiChoco"),
        name: "Choco Ch. 癒月ちょこ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC7fk0CB07ly8oSl0aqKkqFg",
        handle: Some("@NakiriAyame"),
        name: "Nakiri Ayame Ch. 百鬼あやめ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCvzGlP9oQwU--Y0r9id_jnA",
        handle: Some("@OozoraSubaru"),
        name: "Subaru Ch. 大空スバル",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCXTpFs_3PqI41qX2d9tL2Rw",
        handle: Some("@MurasakiShion"),
        name: "Shion Ch. 紫咲シオン",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCp3tgHXw_HI0QMk1K8qh3gQ",
        handle: Some("@YuzukiChoco_Sub"),
        name: "Choco subCh. 癒月ちょこ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UChAnqc_AY5_I3Px5dig3X1Q",
        handle: Some("@InugamiKorone"),
        name: "Korone Ch. 戌神ころね",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCp-5t9SrOQwXMU7iIjQfARg",
        handle: Some("@OokamiMio"),
        name: "Mio Channel 大神ミオ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCvaTdHTWBGv3MKj3KVqJVCw",
        handle: Some("@NekomataOkayu"),
        name: "Okayu Ch. 猫又おかゆ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC1DCedRgGHBdm81E1llLhOQ",
        handle: Some("@usadapekora"),
        name: "Pekora Ch. 兎田ぺこら",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCCzUftO8KOVkV4wQG1vkUvg",
        handle: Some("@HoushouMarine"),
        name: "Marine Ch. 宝鐘マリン",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCdyqAaZDKHXg4Ahi7VENThQ",
        handle: Some("@ShiroganeNoel"),
        name: "Noel Ch. 白銀ノエル",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCvInZx9h3jC2JzsIzoOebWg",
        handle: Some("@ShiranuiFlare"),
        name: "Flare Ch. 不知火フレア",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCl_gCybOJRIgOXw6Qb4qJzQ",
        handle: Some("@UruhaRushia"),
        name: "Rushia Ch. 潤羽るしあ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC1uv2Oq6kNxgATlCiez59hw",
        handle: Some("@TokoyamiTowa"),
        name: "Towa Ch. 常闇トワ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCa9Y57gfeY0Zro_noHRVrnw",
        handle: Some("@HimemoriLuna"),
        name: "Luna Ch. 姫森ルーナ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCqm3BQLlJfvkTsX_hvm0UmA",
        handle: Some("@TsunomakiWatame"),
        name: "Watame Ch. 角巻わため",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCZlDXzGoo7d44bwdNObFacg",
        handle: Some("@AmaneKanata"),
        name: "Kanata Ch. 天音かなた",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCS9uQI-jC3DE0L4IpXyvr6w",
        handle: Some("@KiryuCoco"),
        name: "Coco Ch. 桐生ココ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCAWSyEs_Io8MtpY3m-zqILA",
        handle: Some("@MomosuzuNene"),
        name: "Nene Ch.桃鈴ねね",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCFKOVgVbGmX65RxO3EtH3iw",
        handle: Some("@YukihanaLamy"),
        name: "Lamy Ch. 雪花ラミィ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCK9V2B22uJYu3N7eR_BT9QA",
        handle: Some("@OmaruPolka"),
        name: "Polka Ch. 尾丸ポルカ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCUKD-uaobj9jiqB-VXt71mA",
        handle: Some("@ShishiroBotan"),
        name: "Botan Ch.獅白ぼたん",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCgZuwn-O7Szh9cAgHqJ6vjw",
        handle: Some("@ManoAloe"),
        name: "Aloe Ch.魔乃アロエ",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC6eWCld0KwmyHFbAqK3V-Rw",
        handle: Some("@HakuiKoyori"),
        name: "Koyori ch. 博衣こより - holoX -",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCENwRMx5Yh42zWpzURebzTw",
        handle: Some("@LaplusDarknesss"),
        name: "Laplus ch. ラプラス・ダークネス - holoX -",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCIBY1ollUsauvVi4hW4cumw",
        handle: Some("@SakamataChloe"),
        name: "Chloe ch. 沙花叉クロヱ - holoX -",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCs9_O1tRPMQTHQ-N_L6FU2g",
        handle: Some("@TakaneLui"),
        name: "Lui ch. 鷹嶺ルイ - holoX -",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC_vMYWcDjmfdpH6r4TTn1MQ",
        handle: Some("@kazamairoha"),
        name: "Iroha ch. 風真いろは - holoX -",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCHsx4Hqa-1ORjQTh9TYDhww",
        handle: Some("@TakanashiKiara"),
        name: "Takanashi Kiara Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCL_qhgtOy0dy1Agp8vkySQg",
        handle: Some("@MoriCalliope"),
        name: "Mori Calliope Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCMwGHR0BTZuLsmjY_NT5Pwg",
        handle: Some("@NinomaeInanis"),
        name: "Ninomae Ina'nis Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCoSrY_IQQVpmIRZ9Xf-y93g",
        handle: Some("@GawrGura"),
        name: "Gawr Gura Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCyl1z3jo3XHR1riLFKG5UAg",
        handle: Some("@WatsonAmelia"),
        name: "Watson Amelia Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCq4ky2drohLT7W0DmDEw1dQ",
        handle: Some("@TakanashiKiara_de"),
        name: "Takanashi Kiara SubCh.",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC8rcEBzJSleTkf_-agPM20g",
        handle: Some("@IRyS"),
        name: "IRyS Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC3n5uGu18FoCy23ggWWp8tA",
        handle: Some("@NanashiMumei"),
        name: "Nanashi Mumei Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCgmPnx-EEeOrZSg5Tiw7ZRQ",
        handle: Some("@HakosBaelz"),
        name: "Hakos Baelz Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCmbs8T6MWqUHP1tIQvSgKrg",
        handle: Some("@OuroKronii"),
        name: "Ouro Kronii Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCO_aKKYxn4tvrqPjcTzZ6EQ",
        handle: Some("@CeresFauna"),
        name: "Ceres Fauna Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCsUj0dszADCGbF3gNrQEuSQ",
        handle: Some("@TsukumoSana"),
        name: "Tsukumo Sana Ch. hololive-EN",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCAoy6rzhSf4ydcYjJw3WoVg",
        handle: Some("@AiraniIofifteen"),
        name: "Airani Iofifteen Channel hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCOyYb1c43VlX9rc_lT6NKQw",
        handle: Some("@AyundaRisu"),
        name: "Ayunda Risu Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCP0BspO_AMEe3aQqqpo89Dg",
        handle: Some("@MoonaHoshinova"),
        name: "Moona Hoshinova hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC727SQYUvx5pDDGQpTICNWg",
        handle: Some("@AnyaMelfissa"),
        name: "Anya Melfissa Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UChgTyjG-pdNvxxhdsXfHQ5Q",
        handle: Some("@PavoliaReine"),
        name: "Pavolia Reine Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCYz_5n-uDuChHtLo7My1HnQ",
        handle: Some("@KureijiOllie"),
        name: "Kureiji Ollie Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCjLEmnpCNeisMxy134KPwWw",
        handle: Some("@KoboKanaeru"),
        name: "Kobo Kanaeru Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCTvHWSfBZgtxE4sILOaurIQ",
        handle: Some("@VestiaZeta"),
        name: "Vestia Zeta Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UCZLZ8Jjx_RN2CXloOmgTHVg",
        handle: Some("@KaelaKovalskia"),
        name: "Kaela Kovalskia Ch. hololive-ID",
        branch: Branch::Hololive,
    },
    Channel {
        id: "UC6t3-_N8A6ME1JShZHHqOMw",
        handle: Some("@HanasakiMiyabi"),
        name: "Miyabi Ch. 花咲みやび",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UC9mf_ZVpouoILRY9NUIaK-w",
        handle: Some("@rikka"),
        name: "Rikka ch.律可",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCKeAhJvy8zgXWbh9duVjIaQ",
        handle: Some("@Arurandeisu"),
        name: "Aruran Ch. アルランディス",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCZgOv3YDEs-ZnZWDYVwJdmA",
        handle: Some("@KanadeIzuru"),
        name: "Izuru Ch. 奏手イヅル",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCEzsociuFqVwgZuMaZqaCsg",
        handle: Some("@KagamiKira"),
        name: "Kira Ch. 鏡見キラ",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCGKgJ4MtJ1coi6tWJUfnsQA",
        handle: Some("@YakushijiSuzaku"),
        name: "Suzaku Ch. 薬師寺朱雀",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCANDOlYTJT7N5jlRC3zfzVA",
        handle: Some("@YukokuRoberu"),
        name: "Roberu Ch. 夕刻ロベル",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCGNI4MENvnsymYjKiZwv9eg",
        handle: Some("@KishidoTemma"),
        name: "Temma Ch. 岸堂天真",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCNVEsYbiZjH5QLmGeSgTSzg",
        handle: Some("@AstelLeda"),
        name: "astel ch.アステル",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UChSvpZYRPh0FvG4SJGSga3g",
        handle: Some("@KageyamaShien"),
        name: "Shien Ch.影山シエン",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCwL7dgTxKo8Y4RFIKWaf8gA",
        handle: Some("@AragamiOga"),
        name: "Oga Ch.荒咬オウガ",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCsehvfwaWF6nWuFnXI0AqZQ",
        handle: Some("@TsukishitaKaoru"),
        name: "Kaoru Ch.月下カオル",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCc88OV45ICgHbn3ZqLLb52w",
        handle: Some("@YatogamiFuma"),
        name: "Fuma Ch. 夜十神封魔 - UPROAR!! -",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCdfMHxjcCc2HSd9qFvfJgjg",
        handle: Some("@MinaseRio"),
        name: "Rio Ch. 水無世燐央 - UPROAR!! -",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCgRqGV1gBf2Esxh0Tz1vxzw",
        handle: Some("@UtsugiUyu"),
        name: "Uyu Ch. 羽継烏有 - UPROAR!! -",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCkT1u65YS49ca_LsFwcTakw",
        handle: Some("@HizakiGamma"),
        name: "Gamma Ch. 緋崎ガンマ - UPROAR!! -",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UC2hx0xVkMoHGWijwr_lA01w",
        handle: Some("@AxelSyrios"),
        name: "Axel Syrios Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UC7MMNHR-kf9EN1rXiesMTMw",
        handle: Some("@MagniDezmond"),
        name: "Magni Dezmond Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCDRWSO281bIHYVi-OV3iFYA",
        handle: Some("@NoirVesper"),
        name: "Noir Vesper Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCyxtGMdWlURZ30WSnEjDOQw",
        handle: Some("@RegisAltare"),
        name: "Regis Altare Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UC060r4zABV18vcahAWR1n7w",
        handle: Some("@MachinaXFlayon"),
        name: "Machina X Flayon Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UC7gxU6NXjKF1LrgOddPzgTw",
        handle: Some("@BanzoinHakka"),
        name: "Banzoin Hakka Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCHP4f7G2dWD4qib7BMatGAw",
        handle: Some("@GavisBettel"),
        name: "Gavis Bettel Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
    Channel {
        id: "UCMqGG8BRAiI1lJfKOpETM_w",
        handle: Some("@JosuijiShinri"),
        name: "Josuiji Shinri Ch. HOLOSTARS-EN",
        branch: Branch::Holostars,
    },
];