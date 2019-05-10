package com.example.demo;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public class Utils {
    static String bytesString = "30820c8a30820c31a003020102020101300a06082a8648ce3d04030230123110300e06035504030c074d657361544545301e170d3139303530393033313434385a170d3139303531303033313434385a30123110300e06035504030c074d6573615445453059301306072a8648ce3d020106082a8648ce3d030107034200040dbd3070d9284f1ca1673abb3d526edfe4932dd217cf8d1af6bf5181dabe367c15351712376a06ff5625da6c739420f31d9f10ea8f54245c5111992ea0b677aba3820b7630820b7230820b6e06096086480186f842010d04820b5f7b226964223a223331333234333539303136313731363037343031303833333538393633303832303439343139222c2274696d657374616d70223a22323031392d30352d30395430333a31343a34332e363930363632222c2276657273696f6e223a332c22697376456e636c61766551756f7465537461747573223a2247524f55505f4f55545f4f465f44415445222c22706c6174666f726d496e666f426c6f62223a22313530323030363530343030303930303030303530353032303430313031303030303030303030303030303030303030303030383030303030393030303030303032303030303030303030303030304143413945443239343842344546374445423331413735314145393143424246334544443933433643343033324139303542313431323538344631364131434338373635453039433344414238304542444339393638344531464343423836383331453942423842373930394637313435394342313742413938413642313542333541222c22697376456e636c61766551756f7465426f6479223a2241674141414d6f4b41414149414163414141414141494a7233554d79494a7451705159436e35784e5159346c3479564a77624a354661523541374b76395874504167582f2f7745434141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141427741414141414141414148414141414141414141486e5a636a6349574d565671346f61513846596c76573467502b435249374f574c7a615861335938544649414141414141414141414141414141414141414141414141414141414141414141414141414141414141434431786e6e6665724b4648443275765971545864444138695a32326b434435787737683338434d664f6e674141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414141414e7654427732536850484b466e4f727339556d3766354a4d74306866506a527232763147423272343266425531467849336167622f5669586162484f5549504d646e7844716a31516b584645526d533667746e6572227d7c59756c4b2b614c502b4c345874565239623736423758346764416a59744f4363766d54495164477551313669437230546536766462336f6532322f34734e5a6d3052584668765a32442b5a672b355837674d625a654e414f694b735a4f4b32725155677747502f3456716b585239307531304f37436b66496c5339796c4c504956773574696c6f4d6a6b506d7868576565387a436747656f7959765873423574376b545a78356e485643306f6170344e4e746e6c6679642b693559534e3779534d4f676b7a6e784464614930674b5a7075477532392b5035524d6b6a524c5759345a6b62456a773856683075517749627269346a4a576d6d324c574571374a693677776436547a466f4e394d5a334b5444584a4857757673317274452f39687a67365763687037544c79557251434f4e6e6938735a2f37662f527278694f645a32575878497a4b3478774e2b644469445155557058773d3d7c4d4949456f54434341776d67417749424167494a414e4548646c30796f3743574d413047435371475349623344514542437755414d483478437a414a42674e5642415954416c56544d517377435159445651514944414a44515445554d424947413155454277774c553246756447456751327868636d4578476a415942674e5642416f4d45556c756447567349454e76636e4276636d4630615739754d5441774c675944565151444443644a626e526c6243425452316767515852305a584e305958527062323467556d567762334a3049464e705a323570626d6367513045774868634e4d5459784d5449794d446b7a4e6a55345768634e4d6a59784d5449774d446b7a4e6a5534576a42374d517377435159445651514745774a56557a454c4d416b474131554543417743513045784644415342674e564241634d43314e68626e526849454e7359584a684d526f77474159445651514b4442464a626e526c6243424462334a7762334a6864476c76626a45744d437347413155454177776b535735305a57776755306459494546306447567a644746306157397549464a6c6347397964434254615764756157356e4d494942496a414e42676b71686b6947397730424151454641414f43415138414d49494243674b434151454171586f74344f5a75706852386e75644672414669614778786b676d612f45732f42412b74626543545552313036414c31454e635741344658334b2b453942424c302f375835726a356e4967582f522f317562686b4b5777396766715047334b654174496463762f75544f3179587635307671615076453143524368767a64532f5a45427151356f56764c54505a3356456963516a6c79744b674e39634c6e7862777475764c554b3765795250664a572f6b7364644f7a50385642426e696f6c596e524344326a724d525a386e424d325a5759776e586e7759654f4148562b5739744f6841496d7752774b462f393579417356776432317279484d4a426347483730714c61675a37547479742b2b714f2f362b4b41584a754b775a716a526c457453457a38675a51654666565967637753666f39366f534d417a56723756304c364853444c526e70623678786d625064714e6f6c3474514944415141426f34476b4d4947684d42384741315564497751594d426141464868446533616d66727a51723335434e2b733166447548415645384d41344741315564447745422f775145417749477744414d42674e5648524d4241663845416a41414d474147413155644877525a4d466377566142546f46474754326830644841364c793930636e567a6447566b63325679646d6c6a5a584d75615735305a577775593239744c324e76626e526c626e517651314a4d4c314e48574339426448526c6333526864476c76626c4a6c6347397964464e705a323570626d64445153356a636d77774451594a4b6f5a496876634e4151454c425141446767474241476349746874634b394956527a347252712b5a4b452b376b35302f4f7855736d57386161764f7a4b62306943783037595139727a69356e553733744d45327947524c7a6853566946732f4c704661396c70514c364a4c316151776d4452373454785947424149693566344935544a6f4343457152487a39316b7047365576796e32744c6d6e49644a62504534765976574c72745858664642535350443441666e372b332f58556767416c63376f4354697a4f666262744f466c59413467354b63596753314a325a41654d51716255645a73655a4363615a5a5a6e363574647165653855585a6c447678302b4e644f304c522b357046792b6a754d307757627535394d767a636d5458626a7369374859367a6435335971354b32343466774648525138654f42304957422b3450664d3746654141705a766c66716c4b4f6c4c635a4c327579566d7a526b79523579573732756f396d65685834344369504a32667365395936655174636645684d506b6d4858493031734e2b4b775062704133392b784f7353746a6850394e3159316132745141566f2b7956674c67563248777337334663306f3377433738715045412b76326152732f4265335a46446744796768632f316667552b37432b50366b62716434706f7962364957384b434a6278664d4a766b6f72644e4f674f5555786e64504845692f74622f5537754c6a4c4f6750413d3d300a06082a8648ce3d0403020347003044022056d238023f7586009bdb52c0890e15dc714ba86a8a18dc935db1a78e3023d41902207ad533de3b04e16599a5ec776499cd8cb9a0ea0e29f5cd6989ad232270e2c2ec";

    public static List<Byte> string2BytesList(String[] strings){
        ArrayList<Byte> arrayList = new ArrayList<Byte>();
        for (int i=0;i<strings.length;i++){
            int intVal = Integer.decode(strings[i]);
            arrayList.add(Byte.valueOf((byte)intVal));
        }
        return arrayList;
    }

    public static int getIndexOf(List<Byte> b, List<Byte> bb)
    {
        if (b == null || bb == null || b.size() == 0 || bb.size() == 0 || b.size()<bb.size())
            return -1;

        int i, j;
        for (i = 0; i < b.size() - bb.size() + 1; i++)
        {
            if (b.get(i) == bb.get(0))
            {
                for (j = 1; j < bb.size(); j++)
                {
                    if (b.get(i+j) != bb.get(j))
                        break;
                }
                if (j == bb.size())
                    return i;
            }
        }
        return -1;
    }

    public static byte hexToByte(String inHex){
        return (byte)Integer.parseInt(inHex,16);
    }

    public static List<Byte> convertByte(){
        String certByteString = Utils.bytesString;
        List<Byte> certBytes = new ArrayList<Byte>();
        for(int i=0; i < certByteString.length()/2; i++){
            int m = i * 2 + 1;
            int n = m + 1;

            int intVal = Integer.decode("0x" + certByteString.substring(i * 2, m) + certByteString.substring(m, n));
            certBytes.add(Byte.valueOf((byte)intVal));
        }
        return certBytes;
    }

    public static byte[] list2array(List<Byte> list){
        byte[] bytes = new byte[list.size()];
        for (int i=0;i<list.size();i++){
            bytes[i] = list.get(i);
        }
        return bytes;
    }


}

