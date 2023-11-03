use rand::{thread_rng, Rng};

pub struct Status{
    pub min:u16,
    pub maj:u16,
    pub spe:u16,
    pub num:u16
}
impl Status{
    pub fn new()->Status{
        return Status { min: 0, maj: 0, spe: 0, num: 0 }
    }
}

pub struct Char{
    pub min:Vec<char>,
    pub maj:Vec<char>,
    pub spe:Vec<char>,
    pub num:Vec<char>
}
impl Char{
    pub fn new()->Char{
        let min = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
        let maj = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
        let spe = vec!['.',',',':',';','?','!','@','&','#','$','%'];
        let num = vec!['0','1','2','3','4','5','6','7','8','9'];

        return Char{
            min: min,
            maj: maj,
            spe: spe,
            num: num
        }
    }
}

pub fn make_mdp(r:u16)->String{
    let d = Char::new();
    let mut rng = thread_rng();
    let mut mdp:String = Default::default();
    let mut s = Status::new();

    for x in 0..r{
        let control: u16 = s.min + s.maj + s.spe + s.num;
        let c:i8 = x as i8 - control as i8;
        let c1:u8;
        if c <= 0{
            if s.min == 0{c1 = 0;}
            else if s.maj == 0{c1 = 1;}
            else if s.spe == 0{c1 = 2;}
            else if s.num == 0{c1 = 3;}
            else{
                c1 = rng.gen_range(0..4);
            }
        }
        else{
            c1 = rng.gen_range(0..4);
        }
        let choice: &Vec<char>;

        match c1{
            0=>{
                s.min+=1;
                choice = &d.min;
            },
            1=>{
                s.maj+=1;
                choice = &d.maj;
            },
            2=>{
                s.spe+=1;
                choice = &d.spe;
            },
            3=>{
                s.num+=1;
                choice = &d.num;
            },
            _=>panic!("Une erreur est survenue.")
        }

        mdp += &choice[rng.gen_range(0..choice.len())].to_string().as_str();
    }
    return mdp
}