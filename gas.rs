//gas.rs
//by firebystream
//2026.10.2

const R :f64 = 8.314;

const r :f64 = 8.314;

struct IdealGas{
    p:f64,
    v:f64,
    n:f64,
    t:f64
}


impl IdealGas{
    fn new(mut p:f64,mut v:f64,mut n:f64,mut t:f64)->Self{
        if n==0.0{
            n = p * v / (t*r);
        }else if v==0.0{
            v = r*n*t/p;
        }else if t==0.0{
            t = p * v / (n*r);
        }else{
            p = r*n*t/v;
        }

        Self{p,v,n,t}
    }
}




fn scinum(mut a:f64,mut exp:i32) ->f64{
    if exp > 0{
        for i in 0..exp{
            a*=10.0;
        }
    }
    
    if exp < 0
    {
        exp = -exp;
        for i in 0..exp{
            a*=0.1;
        }
    }
    a
}

fn main(){
    println!("Welcome to use gas.");

    let a = IdealGas::new(scinum(1.33,-5),scinum(5.0,-2),0.0,300.0);

    println!("answer1: {} \n\n",a.n * scinum(6.0221367,23));
  
    let b= IdealGas::new(0.0,0.02,220.0/44.0,400.0);

    println!("answer2: {} \n\n",b.p);

    let c = IdealGas::new(scinum(1.01325,5),2.5,0.0,553.0);

    println!("answer3: {} \n\n",c.n*86.94);

    let dA = IdealGas::new(scinum(6.0,4),scinum(1.25,-4),0.0,300.0);
    let dB = IdealGas::new(scinum(8.0,4),scinum(1.5,-4),0.0,300.0);

    let n_A_and_B = dA.n + dB.n;

    let dmix = IdealGas::new(0.0,scinum(5.0,-4),n_A_and_B,300.0);

    println!("answer4: {}\n\n",dmix.p);

}