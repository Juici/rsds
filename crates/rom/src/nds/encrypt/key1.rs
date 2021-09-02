use byteorder::{ByteOrder, LittleEndian};

const KEY_DATA_LEN: usize = 0x412;

/// Encryption key data from ARM7 BIOS.
///
/// Sourced from `0x0030..0x1078`, and interpreted as `u32`s with little-endian encoding.
static KEY_DATA: [u32; KEY_DATA_LEN] = [
    0x5F20D599, 0xB9F54457, 0xD9A4196E, 0x945A6A9E, 0xEBF1AED8, 0x3AE27541, 0x32D08293, 0xD531EE33,
    0x9A6157CC, 0x1BA20637, 0xF5723979, 0xBEF6AE55, 0xFB691B5F, 0xE9F19DE5, 0xA1D92CCE, 0xE605325E,
    0xCFFED3FE, 0x0D0462D4, 0xB7ECF58B, 0xBB79602B, 0x0D319512, 0x2BDA3F6E, 0xF1F08488, 0x257E123D,
    0xBBF12245, 0x061A0624, 0x28DFAD11, 0x3481648B, 0x2933EB2B, 0xBDF2AA99, 0x9D95149C, 0x8CF5F79F,
    0x29A19772, 0xCF5FD19D, 0x1A074D66, 0x4B4AD3DE, 0xA3A7C985, 0x3A059517, 0xBF0A493D, 0xA28B890A,
    0xDD49824A, 0x0BF19027, 0x6A1CEBE9, 0x05457683, 0x617081BA, 0xDE4B3F17, 0x39ABCFAE, 0x563AF257,
    0x8AAD1148, 0x3F45E140, 0x54029BFA, 0xFB93A6CA, 0x6FFE4DEF, 0x9C87D8A3, 0x48D5BA08, 0xFD2D8D6A,
    0x74F8156E, 0x8B52BEBD, 0x9E8A2218, 0x073774FB, 0x4A6C361B, 0x6242BA19, 0x109179B9, 0x9665677B,
    0xE82302FE, 0x778C99EE, 0x64865C3E, 0x86786D4D, 0xE2654FA5, 0x5ADFB21E, 0x087ED00A, 0xAC71B014,
    0x1C83DBBD, 0x62A1D7B9, 0x7C63C6CD, 0xE6C36952, 0x12CE75BF, 0x04215D44, 0x3CD3FBFA, 0xD4631138,
    0x49418595, 0x08F20946, 0x1FDC1143, 0x6D15C076, 0x70633C1F, 0x6C8087EA, 0x8B63BDC3, 0x372137C2,
    0x2309EEDC, 0x4D6A372E, 0x50F79073, 0x921CAC30, 0x91231004, 0xAA07D24F, 0x9A4F3E68, 0x6A6064C9,
    0xF32114C8, 0x124122D6, 0xE6CF2444, 0x0DDD568A, 0x85E14D53, 0x5A528C1E, 0xC284199C, 0x6FF15703,
    0x58BE00E3, 0xD5ED4CF6, 0x1F9C6421, 0x3C0355BE, 0xAAFFDC4A, 0x5DE0DAC9, 0xDEE6BF5E, 0xF8B1D8F5,
    0xB9B336FF, 0xDB956762, 0xED375F31, 0x9967704C, 0x3118B590, 0x99993D6C, 0xD3DA42E4, 0xA0134225,
    0x6C70D7AE, 0xC7CF55B1, 0x43D546D7, 0x443D1761, 0x8533E928, 0x93A2D0D5, 0x1F1225AA, 0x460BC5FB,
    0x567697F5, 0x87BEA645, 0xE86B94B1, 0x9933FEB1, 0x6C3E1FAE, 0x091D7139, 0xE4379000, 0x74753E10,
    0x3B838CFF, 0xF9B0F1B0, 0x42470501, 0xACD6F195, 0x9EE6387E, 0x3F267495, 0x185068B4, 0xB43043D0,
    0x68E34B4C, 0xB64DE5BF, 0xA00A8B95, 0x77322574, 0x2CF7A1CF, 0x5A1371D8, 0x51C9EAAB, 0xEFEE0DE8,
    0x197E93E9, 0x38431EA7, 0xA12C1681, 0xCC73E348, 0xD36C2129, 0xD9A0CE5D, 0xA0437161, 0x64B51315,
    0x192ACF92, 0xA5B7ADDC, 0xF865869F, 0xFBE79F1A, 0x13B8FDF7, 0x6FDB276C, 0xF71C35DF, 0x9B5B2C8D,
    0x6438AB12, 0x31DECC06, 0x11754EE8, 0xEAFAE364, 0xC25434EB, 0xEB343FAD, 0x267D2C93, 0xF3569D36,
    0xB3F6E15A, 0x9E4A6398, 0x9AE48332, 0x907D6084, 0xEE0E132E, 0xA2364B93, 0x3816EC85, 0x020688E8,
    0x3AA0F0BF, 0x9A6AD7ED, 0xCF57E173, 0xDCB844F8, 0xD159232E, 0x715295DF, 0x4BA06199, 0x786E7FD5,
    0x30C5A9BA, 0x328640D3, 0x9C0C329D, 0x2F02B737, 0xA99854BA, 0xC90413C4, 0xE7C8BE8D, 0x2E50975D,
    0x5922D693, 0x22BC270C, 0x20A7E092, 0x7F6F930F, 0xB5D39F4C, 0x740B2AA6, 0x107D4967, 0xC5D1CB26,
    0x8CE77186, 0x5BE99CA0, 0x01F61AB2, 0x5E9E8CEE, 0xDB1AF283, 0x84EAE5E6, 0x7CD27659, 0x49A58DF6,
    0x16C24836, 0xA383BB52, 0x0C07B974, 0x2861FF3B, 0xE4E961E1, 0xAA156EEF, 0x5DE8BA4E, 0x32BB9605,
    0x72FBB056, 0xC80E0F52, 0x76652542, 0xDEF2AF89, 0x01F02710, 0x97A7744B, 0x5426D507, 0x821F0954,
    0x307D860A, 0x26B30E39, 0xBB570B9B, 0xAF310636, 0xD9FC79FD, 0x0C2B1030, 0xD79BE1B3, 0xEF5FDC7B,
    0x4513F8D2, 0xBD75474D, 0x7E3C9646, 0xB53EF375, 0x3B9AC567, 0x6B295BB0, 0xC85B80DE, 0x31B10515,
    0xDD49CEB6, 0xAEB584AD, 0x3167DC60, 0x4EFE3034, 0xA62F80BD, 0x213963BF, 0x7F35D986, 0x05226816,
    0x2690E954, 0x516C078C, 0xD75531A4, 0x3EA80709, 0xC166532E, 0xC47BF2F8, 0xF1CF58F2, 0xE7A2C587,
    0x87308F27, 0x6264A058, 0x88B91823, 0xC4CEFA7C, 0x17ADAE98, 0xF35B4ACC, 0x56D548E9, 0xC8F20DD3,
    0xDB8C7392, 0xAC562FD7, 0x6992F981, 0xF632C64D, 0x218DC0E6, 0x618076E2, 0x6CDCBC11, 0x6919AF93,
    0xB9BFD09B, 0x67029F31, 0x83EE51A3, 0x0C7B2206, 0x404249AB, 0x7D01D5B8, 0x55F75ECE, 0x99C53953,
    0x9F87D846, 0xB464F7BA, 0xA1FA9AE3, 0x1068906D, 0x548ACA30, 0xC3609FA7, 0x0D6BF519, 0xE698517A,
    0xB4514398, 0x4FE935D6, 0x7B0FDFC3, 0xBD5C2FD6, 0x1961153A, 0xAACB4BF1, 0xC9646DDC, 0x561EC6D3,
    0x504C38EF, 0xCC758671, 0xE94E0D0D, 0x5D06F628, 0xD3AA1B70, 0x39A8CF45, 0x2EA695AC, 0xD422E4B4,
    0x5F37A874, 0xCC047A48, 0xD8404CA5, 0x0828B428, 0x52721C0D, 0x477DF041, 0x4E533A19, 0x6B628458,
    0x818AB593, 0xDC0D4E21, 0xC6A23FB4, 0x402BC9FC, 0xE90438DA, 0x6B865A5E, 0x8525220C, 0x7C8D1168,
    0x55951D92, 0xBB8EAB4D, 0xB7E6A6DA, 0x5A32B651, 0x05DD4105, 0x50560A2A, 0xCC471791, 0xB57EE6C9,
    0x73DB4A61, 0x33C85167, 0x746EDAF5, 0x37C3542E, 0x08AF6D0D, 0x5F8A15E8, 0xCD2159E2, 0x060CDEA8,
    0x5F6B775A, 0x3E6518DB, 0x78DE50C8, 0xB382B8E0, 0x32724E5D, 0x34C14F07, 0xB796BA23, 0x28A44E67,
    0xEB62341E, 0xE9706A2D, 0x70C4422F, 0x9C315A4E, 0x28475BF9, 0x6F71DAAA, 0x78B31F38, 0x1C6B92C4,
    0x9A35F69E, 0xBF0E4DB7, 0x412918CC, 0x5D354803, 0xC62BD055, 0x605CAF29, 0x5E8E6974, 0xBDD47C9B,
    0x7D64447B, 0x695D923F, 0x4B001FB6, 0xCF3583D4, 0x174E647E, 0x2ED58DAE, 0x4E12289A, 0x08492B2E,
    0x46C6AE5C, 0x6141AE85, 0xD2826F1E, 0x1F163751, 0xA459F60B, 0xAF5ACA9A, 0x8B33D40D, 0x84F16320,
    0xCFCB5C80, 0xD3B9B408, 0x62BD0516, 0x569B3183, 0xBA9F9851, 0xB2AA5BB2, 0xB52C6B22, 0x63FA48D4,
    0xFA585F2B, 0x0964FA61, 0xB8E038BB, 0xA860929D, 0x0E6F670D, 0x010DF537, 0xD477C29F, 0x73F1ECFE,
    0x7DE03930, 0xE49861F5, 0x0455282C, 0x2FDB5556, 0x58E5EC6B, 0x8064B606, 0x4E1A2A6A, 0xC4D80F5B,
    0x19522E0A, 0x30F562D9, 0x7B8CBE48, 0xA29B384F, 0xD3C9AFC3, 0x4162C1C7, 0x2161B986, 0x4F996F57,
    0x7BCEBAC1, 0x5E4D3BB5, 0x57448B8A, 0x705F135F, 0x47295B6D, 0xECE238DC, 0x12655504, 0x4317E82A,
    0x2ADD8EE1, 0xF794E2B3, 0xE65C6E09, 0x6DF88AEB, 0x48544989, 0xBFAD2FF5, 0xCA4B94EA, 0x828739FC,
    0xF2018A5F, 0x71E6F275, 0xDE42D8D6, 0x281D2DF1, 0xA37E88A6, 0x301D47A0, 0xDF71A3D9, 0x01CB1C49,
    0xF2B136F8, 0x5D5822F0, 0xA0BD6B45, 0x4288B2BB, 0xCE288CC7, 0x6390E893, 0x897C9008, 0xB77DF53C,
    0x554F2D04, 0x7EFD1651, 0xC1BEE879, 0xF8D412F2, 0x230584B4, 0x2BD2CCA0, 0xADABE1FD, 0x6C55D10D,
    0x4D944123, 0x054F3777, 0x17BF0C28, 0x6C6712B3, 0xF75AC38C, 0x6D2A8441, 0x271294D0, 0x9CEDB42C,
    0x8247EC4D, 0xB967D597, 0x55C09D1B, 0x8EE57E07, 0x3EE7A8E2, 0x3A0EE412, 0x3455452A, 0x5A2DF9A2,
    0x7C52AB1B, 0x555F1083, 0x435AF1D2, 0xA4A7C62B, 0xE8951589, 0xF89D4BB4, 0x609FE375, 0xE6D65B78,
    0x21E6440D, 0x2247BD06, 0xAD00A453, 0x8513438D, 0xFCAAF739, 0xED7BAF38, 0x542BE4FC, 0xFC4C9850,
    0xDFF78085, 0xE122803C, 0x24DEDA94, 0x397AB0C6, 0xA10FDC38, 0x6FF9F4A7, 0x8B571863, 0x2E2A4184,
    0xD9F253D4, 0xDDD00F00, 0xA6196E99, 0x5BECD00A, 0xC0AB2458, 0xEC6506CB, 0x9438131A, 0x2F03670A,
    0x77E3F73F, 0xC6337744, 0xE3D03914, 0x7908A2C0, 0x579940BB, 0x90010B41, 0x48CCE1CD, 0xAFB3DB67,
    0x4CF37488, 0xB1728F82, 0xC42923B5, 0xFC196C12, 0x9CA4468E, 0x876525C4, 0x8ABE6DD3, 0x38031193,
    0xF32B83ED, 0xEA93A446, 0x1D85533B, 0x08F1D4CE, 0xFCED2783, 0xBC181A9B, 0xDCAE8BF9, 0x3850AB24,
    0x104B72E9, 0x467B1722, 0x6459AB5D, 0xF8AE40F3, 0xF9C8E5BB, 0x554E0326, 0xFEEBEB7D, 0xE0E639F7,
    0x2EBE110A, 0xED98FF28, 0x5642C9C0, 0x00FDC342, 0xA287AFF6, 0x323F015B, 0x9A954792, 0x3D32A572,
    0x9BD06BAE, 0x9249D207, 0xFA4A78E3, 0xF27D06A1, 0x7477CF41, 0x0CB21404, 0x16648486, 0xA151BBD5,
    0xD1F16FE5, 0x5FF7E2F2, 0xB84D2058, 0xDDCFC757, 0x76BED8C5, 0x7E5FF63D, 0x888B2AE7, 0x3F381B24,
    0x7723410E, 0xD44BF0F5, 0xA4FA1F0C, 0xCF5F800B, 0xDAE0F645, 0x5359342F, 0x523C20FB, 0xB5355E62,
    0x608BFE62, 0x5A86E363, 0xD16E1A15, 0x32BC4547, 0x3867EBB4, 0x336EE4AB, 0xA3EDB53A, 0x4EE067AD,
    0x62EE9541, 0x1D267162, 0x3062EF31, 0xAC82D7AF, 0x0405DCC2, 0xBF0797F5, 0x07235911, 0xE80264C0,
    0xAF3EE597, 0xA659AC18, 0x90334A8B, 0x9C7C6E1C, 0x3C4C7E20, 0xBB64613E, 0x7E7C6BC5, 0x4CC59F3E,
    0xF573EA9F, 0x4CC089D7, 0x2DF4FBF4, 0x511B14EC, 0xC812C1D5, 0x4A0BDF10, 0x93BC9C8B, 0x3E3E6A45,
    0xBAA9C17D, 0x07B4C1CD, 0x8668E1E4, 0x386DB243, 0x5C0CFBF3, 0xDE713766, 0xA06EEF56, 0xA7654010,
    0xBED0F798, 0x3637C80E, 0x7CCA10EC, 0x1E84AB9C, 0x02761705, 0xAA524F1C, 0xA0C6C15F, 0x04D8B956,
    0xA74D4484, 0x60DED859, 0x050E38E6, 0x3BE1038F, 0x3304816D, 0xCE0B306F, 0x33210569, 0x89BB26FB,
    0x87AEB67D, 0xE007517E, 0x0A96F7AC, 0x5CC4F96B, 0x4744E41D, 0xE3FA5EB8, 0x42558478, 0xF75E484B,
    0x8635477D, 0x05432B1D, 0xB88AEC03, 0x763C061E, 0x431A480C, 0xED8AB7A7, 0x43C6131E, 0xDBEF10EE,
    0x833CFBEC, 0xEF4495B2, 0x4E5154D8, 0x1D44112D, 0x1E5936FB, 0xC3C1347A, 0x610057CA, 0x16A567EA,
    0x55D0559B, 0x36D97FE1, 0xAE7640D2, 0xB0CE01DC, 0xCBD5837A, 0x6BEC9820, 0x349272C1, 0x375782F3,
    0x36328A62, 0xAE43900C, 0x789B5CAE, 0x0265138E, 0xC17168FD, 0xA031B0FE, 0xC3B08224, 0xA76979B1,
    0xD0EBD2F5, 0xDC32C082, 0x3C26C79E, 0xC1988D6D, 0xD0D422BB, 0x3EEC330F, 0xDCE1CCB9, 0x36774C6A,
    0xBFF91C14, 0x5F289F81, 0x29328571, 0xC4487590, 0xD8CE4AB3, 0x2F148F44, 0xEF5740FD, 0xD97508AA,
    0x6ED6D146, 0xC31F5532, 0x1F84FE18, 0xFFD584FC, 0x481B5E71, 0x0E9586C3, 0xD3270828, 0x7B718338,
    0x5463804C, 0xACB0569A, 0x31CA80CF, 0xF3FEEF09, 0x7E24AFBE, 0x3F53FEA6, 0x334A8DC2, 0xA622D168,
    0xEA7BAD66, 0xB043B6DE, 0x009525A1, 0x46753FA3, 0xEC441114, 0x92BC95D7, 0x16A94FF0, 0x60976253,
    0xF1410F2A, 0xEEBE2471, 0xCD087F94, 0x85B39360, 0x3F00075B, 0x83280FD8, 0x9F69D19A, 0xC32EDAD1,
    0xB9A20190, 0x662A4E6B, 0xA6AEDA9D, 0x68D32AEA, 0x9C0C0C2F, 0xED4A8CD2, 0x65579EE2, 0xA387099D,
    0x5D32C4B4, 0x2B32D4C9, 0x1E71E0B1, 0x90E64D64, 0x401EE371, 0x84F37DED, 0x78C8ED0E, 0x71C0AE76,
    0x05BB7227, 0xFB6402EA, 0xB56B48F3, 0xED3F9342, 0xD253139F, 0xEC2AFEF7, 0xDB25471D, 0xC686913C,
    0xFD11F08E, 0xF7367423, 0x7A9EF5A4, 0x4450537E, 0xD3CA47D4, 0xE66D38EB, 0x7F9471D9, 0x4B69C64A,
    0xEA52F411, 0xB08AFE22, 0x598B6736, 0x2A80E6E8, 0x130465EB, 0x9EDCECEE, 0x05ECB15F, 0x9FE6596A,
    0x896B595E, 0xCA1AF7BF, 0x6A5BF944, 0xE4038571, 0x70E06229, 0xCFC4416F, 0xE3CCB1B2, 0xA807A67E,
    0x847FE787, 0x4B52DB93, 0xDD7EEC6C, 0x104824D4, 0x60049F69, 0x1848E674, 0xB92CE4F3, 0x7A502E4F,
    0x6954D4DF, 0xF3A78B2B, 0xF31FFFCE, 0x3901263E, 0x89849517, 0x4B4CF0B0, 0xC49F9182, 0xA59DAC4B,
    0x2517AF74, 0xD332CAC9, 0x848A89BC, 0xAE0DCC89, 0x9CDBA27C, 0xEE91786A, 0x4E5D76EA, 0x69F56087,
    0x02D46715, 0x3648AFCF, 0x6FBFEA07, 0x8F062D66, 0xF9FE9AC4, 0x758790F6, 0x0FADF7B8, 0x3D5A1076,
    0xB32EB059, 0xCC2C35C7, 0xCB2B5670, 0xC59637E3, 0x8A1B462F, 0x88C74622, 0x983226A7, 0x2286DF61,
    0x2F1CF48A, 0xAA09A187, 0xD3AEA9CC, 0x1C4500BD, 0x8687549A, 0xFFEF8752, 0x8FA18F1E, 0x355C89C1,
    0x3A2DDA1B, 0xC2B2162C, 0x78E256F1, 0x97636BC1, 0xC98F56C5, 0xAA2C7F32, 0xACA8A6AF, 0x88229120,
    0x8B60E4DE, 0x25424BF9, 0x9C7FE31A, 0x3A89192C, 0x36D4057E, 0xC25869CC, 0x2F8B32C1, 0x7AEB8590,
    0xA1A55039, 0x66C59227, 0x584F20B0, 0x4383557E, 0x9CE2452B, 0x9012D8E4, 0x5683162C, 0xB3037916,
    0x18612DAD, 0x371F131A, 0x739CE1E2, 0xFDD5807B, 0xFC87512D, 0x1FD7AA7B, 0xAF8E7A2C, 0xCDBB8DF4,
    0x727C1195, 0xE26FEE0B, 0x37DEAFB9, 0x8D8CDE83, 0xB7670562, 0x568DC696, 0x62D70DB6, 0x3646D6BA,
    0xE6C88EBD, 0x106C2AEA, 0x5B6BFF14, 0x463C82FA, 0x464330B1, 0x9B7D8A51, 0x79833E92, 0xB25D555B,
    0x90CE5E6C, 0x98538E62, 0xE56D0DC9, 0xC5CD572D, 0xE1BA5781, 0x728FB8E8, 0xDC134FE5, 0x15719DEA,
    0x8811B210, 0x7FD409D5, 0x2C7F655B, 0x114C383B, 0xFB8D5068, 0xBF59B09E, 0x4A898094, 0x12181AC5,
    0x4AD15389, 0x8CE82910, 0xEAB6EC1C, 0x8B17C746, 0xA8311525, 0xB1436BA2, 0x0BDBE29D, 0x11B09B87,
    0xD2710E04, 0x82897729, 0x7F41660A, 0xFF480B1D, 0xFD24BB72, 0x9BA148C2, 0xCE7F7BFE, 0xD986DB88,
    0xB01C3B85, 0x0733A8DC, 0xE32E51BF, 0x97009A0E, 0x97C0061E, 0xB6D89D43, 0x6786C445, 0x88F8005F,
    0x9E52A49A, 0x838AAAC7, 0x18C5EC75, 0x2FC3CEAE, 0x18F92B1A, 0xF51AAEFF, 0x33B50B53, 0xE8FDA751,
    0x64A2E1A8, 0x431722B6, 0xD80ACC80, 0x40BA3BAE, 0x4A92D9D7, 0x1004DF89, 0x2B189BEE, 0x8A69776A,
    0xB9F9F468, 0x6E1521A2, 0x033B1EE6, 0x609B3062, 0x9B257E41, 0x52C58F9E, 0xC2F80810, 0x1121A169,
    0x795E3788, 0x10FF6635, 0xED6E1842, 0x1C6BB697, 0x6DE5364E, 0xBFE4B47D, 0x05E0B920, 0xB8D5693A,
    0xE0DCD5E3, 0x3E53ACB9, 0xAD57A407, 0x1848FF77, 0x49AC2A76, 0x75478E2A, 0x63679F6D, 0x398C3530,
    0x6FD53905, 0xAD5B3A64, 0x82BB0BCA, 0xB1459952, 0x99363693, 0x442013AF, 0x4402D836, 0x85923909,
    0x974A4AFF, 0xD763A687, 0x24B5B5C7, 0x6FB40FED, 0x1452580C, 0xD37BA6D9, 0x5838BC79, 0x843BBDA1,
    0x061AD806, 0xEAA86BFD, 0x0428694B, 0x9982AD37, 0x851B0EFB, 0x735DA8BD, 0x7558DCCD, 0x6C63BE0A,
    0xE44CE748, 0x60042B30, 0xDAD815B9, 0x8F758186, 0x1C8DD496, 0x7C85705D, 0xD57B671C, 0xCEA66708,
    0x70660A4B, 0xD463E5B7, 0xEA828A5B, 0xE2CA6710, 0x8517EFF4, 0x8A5F2A2F, 0x6AF88297, 0xEA1034D6,
    0x3C5CC9EB, 0x46F849E1, 0xF6BDDEEB, 0xAAF192A9, 0xB018A0A6, 0x1F0FD33A, 0x31FF6FF3, 0xD3444345,
    0x88F79A50, 0xCEC19609, 0x2CF2CC76, 0x82ADBA2C, 0x84188F77, 0x9C07D2C0, 0x4E839036, 0x434FA50B,
    0x78AB043E, 0x09FBD64F, 0xDA902401, 0x613A3C6F, 0x4A697F0D, 0x02302BEB, 0x84E0DBB4, 0x35D7ECA9,
    0x857D37BF, 0x4EA9CE58, 0xA8C780E4, 0x486730D3, 0x2FAF29EB, 0xA7B46A74, 0x923F0F3F, 0xACCAF3AF,
    0x94D94BAF, 0x81CA43C0, 0xA1482F0D, 0xD2D527B0, 0x85054BEF, 0x934DDEA3, 0xBBF03C30, 0x27308F4A,
    0x3EE3EB4C, 0x2F9AED64, 0xF082F13B, 0x7FCFF4BA, 0xE1B0CB40, 0x57AABC7F, 0xF274C9D3, 0x220D43FA,
    0x4E77F4D0, 0x7085D793, 0xB6BF991F, 0x30F135DE, 0xF0715EA7, 0x7B2D016B, 0x5333F064, 0xF388390A,
    0x6BA63A6B, 0x432FD235, 0xB5FD02CD, 0xAA5BBCE9, 0x7E19A4D8, 0x81945D0E, 0xAD776F9E, 0x93740ED6,
    0x18C4E796, 0x19F5AD5F,
];

#[derive(Debug)]
pub struct Key1 {
    // This holds both the `p` and `s` used in the blowfish algorithm.
    //   p    = key_buf[0x000..0x012]
    //   s[0] = key_buf[0x012..0x112]
    //   s[1] = key_buf[0x112..0x212]
    //   s[2] = key_buf[0x212..0x312]
    //   s[3] = key_buf[0x312..0x412]
    key_buf: [u32; KEY_DATA_LEN],
}

impl Key1 {
    fn lookup(&self, x: u32) -> u32 {
        let mut a = (x >> 24) & 0xFF;
        let mut b = (x >> 16) & 0xFF;
        let mut c = (x >> 8) & 0xFF;
        let mut d = (x >> 0) & 0xFF;

        a = self.key_buf[(a as usize) + 0x12 + 0x000];
        b = self.key_buf[(b as usize) + 0x12 + 0x100];
        c = self.key_buf[(c as usize) + 0x12 + 0x200];
        d = self.key_buf[(d as usize) + 0x12 + 0x300];

        (a.wrapping_add(b) ^ c).wrapping_add(d)
    }

    fn expand_key(&mut self, key: &[u32; 3]) {
        for i in 0x0..0x12 {
            // We only use modulo 2 at the moment.
            self.key_buf[i] ^= key[i & 1].swap_bytes();
        }

        let mut lr = (0, 0);
        for i in 0x0..0x9 {
            lr = self.encrypt(lr.0, lr.1);
            self.key_buf[2 * i] = lr.1;
            self.key_buf[2 * i + 1] = lr.0;
        }
        for i in 0x0..0x200 {
            lr = self.encrypt(lr.0, lr.1);
            self.key_buf[0x12 + 2 * i] = lr.1;
            self.key_buf[0x12 + 2 * i + 1] = lr.0;
        }
    }

    fn encrypt(&self, mut l: u32, mut r: u32) -> (u32, u32) {
        for i in 0x0..0x8 {
            r ^= self.key_buf[2 * i];
            l ^= self.lookup(r);
            l ^= self.key_buf[2 * i + 1];
            r ^= self.lookup(l);
        }
        r ^= self.key_buf[0x10];
        l ^= self.key_buf[0x11];
        (r, l)
    }

    fn decrypt(&self, mut l: u32, mut r: u32) -> (u32, u32) {
        for i in (0x1..0x9).rev() {
            r ^= self.key_buf[2 * i + 1];
            l ^= self.lookup(r);
            l ^= self.key_buf[2 * i];
            r ^= self.lookup(l);
        }
        r ^= self.key_buf[0x1];
        l ^= self.key_buf[0x0];
        (r, l)
    }

    fn apply_keycode(&mut self, key: &mut [u32; 3]) {
        let [a, b, c] = *key;
        let (b, c) = self.encrypt(b, c);
        let (a, b) = self.encrypt(a, b);
        *key = [a, b, c];

        self.expand_key(key);
    }

    /// Initialise KEY1 with level 1.
    pub fn init1(game_code: u32) -> Key1 {
        let mut key1 = Key1 { key_buf: KEY_DATA };
        let mut key = [game_code, game_code >> 1, game_code << 1];

        key1.apply_keycode(&mut key);

        key1
    }

    /// Initialise KEY1 with level 2.
    pub fn init2(game_code: u32) -> Key1 {
        let mut key1 = Key1 { key_buf: KEY_DATA };
        let mut key = [game_code, game_code >> 1, game_code << 1];

        key1.apply_keycode(&mut key);
        key1.apply_keycode(&mut key);

        key1
    }

    /// Initialise KEY1 with level 3.
    pub fn init3(game_code: u32) -> Key1 {
        let mut key1 = Key1 { key_buf: KEY_DATA };
        let mut key = [game_code, game_code >> 1, game_code << 1];

        key1.apply_keycode(&mut key);
        key1.apply_keycode(&mut key);

        key[1] <<= 1;
        key[2] >>= 1;
        key1.apply_keycode(&mut key);

        key1
    }

    // TODO: Check encrypt/decrypt block function performance.

    /// Encrypts a block of 8 bytes.
    pub fn encrypt_block(&self, block: &mut [u8]) {
        let l = LittleEndian::read_u32(&block[..4]);
        let r = LittleEndian::read_u32(&block[4..]);
        let (l, r) = self.encrypt(l, r);
        LittleEndian::write_u32(&mut block[..4], l);
        LittleEndian::write_u32(&mut block[4..], r);
    }

    /// Decrypts a block of 8 bytes.
    pub fn decrypt_block(&self, block: &mut [u8]) {
        let l = LittleEndian::read_u32(&block[..4]);
        let r = LittleEndian::read_u32(&block[4..]);
        let (l, r) = self.decrypt(l, r);
        LittleEndian::write_u32(&mut block[..4], l);
        LittleEndian::write_u32(&mut block[4..], r);
    }
}
