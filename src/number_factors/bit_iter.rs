//Iterator used to find all combinations for a vector set V into two disjoint non-empty unique subsets A and B.
//A/B has at least one element, (A U B)=V, and (A Intersect B)=Empty.
//The unique sets of A and B can be interchanged and is counted as 1 only.
//Example: If V={1,2,3}, then A,B={2},{1,3} and A,B={1,3},{2} is counted as 1 only.
//This is valid if size is >=2.
//There is a special case for size 1 for this code, where B can be empty.
pub struct BitCombinationsIter{
    bitcomb:u64, //Number represented as 1s and 0s
    bitcomb_last:u64, //If all bits reached this number.
    bits:usize,
    size:usize, //Size of part of the u64 based on BitCombinationsIter::new(size) 
    init_started:bool, //The first set 1 0 ... 0 0 will not be skipped.
    check_half_last: bool //For an even set, stop iterating where boolean values are flipped.
}
impl BitCombinationsIter{
    pub fn new(size:usize)->Self{
        Self{bitcomb:1u64,
            bitcomb_last:bits_end_mask(size,1usize),
            bits:1usize,
            size,
            init_started: false,
            check_half_last: if size!=2{false}else{true}//2 is 1 0 and 0 1. Only iterate 1 0.
        }
    }
}
impl Iterator for BitCombinationsIter{
    type Item=u64;
    fn next(&mut self)->Option<Self::Item>{
        let bitcomb_half_last=if !self.check_half_last{
            None
        }else{
            Some(self.bitcomb)
        };
        if self.init_started{
            if self.bitcomb!=self.bitcomb_last{
                self.bitcomb=next_bit_lex(self.bitcomb);
            }else{
                if self.bits==1&&self.size==1{return None;} //If SplitVectorIter::size is 1.
                self.bits+=1;
                if self.bits%(self.size/2)==0{
                    self.check_half_last=true;
                }else if self.bits==self.size/2+1{
                    return None;
                }
                self.bitcomb=bits_reach_start(self.bits);
                self.bitcomb_last=bits_end_mask(self.size,self.bits);
            }
        }else{
            self.init_started=true;
        }
        match bitcomb_half_last{
            None=>{
                Some(self.bitcomb)
            }
            Some(last_bc)=>{
                if self.bitcomb==!last_bc&bits_reach_start(self.size){//The next half is just flipped versions of 0s and 1s and backwards.
                    None
                }else{
                    Some(self.bitcomb)
                }
            }
        }
    }
}
fn bsf(n:u64)->isize{//From https://www.youtube.com/watch?v=ZRNO-ewsNcQ
    if n==0{return -1;}
    let nm=n&(-(n as i64) as u64);
    let mut count=0;
    if nm&0b1111111111111111111111111111111100000000000000000000000000000000!=0{count+=32}
    if nm&0b1111111111111111000000000000000011111111111111110000000000000000!=0{count+=16}
    if nm&0b1111111100000000111111110000000011111111000000001111111100000000!=0{count+=8}
    if nm&0b1111000011110000111100001111000011110000111100001111000011110000!=0{count+=4}
    if nm&0b1100110011001100110011001100110011001100110011001100110011001100!=0{count+=2}
    if nm&0b1010101010101010101010101010101010101010101010101010101010101010!=0{count+=1}
    count
}
fn next_bit_lex(n:u64)->u64{//From https://www.youtube.com/watch?v=ZRNO-ewsNcQ
    let t=(n|(n-1)) as i64;
    ((t+1)|((!t&-(!t))-1)>>(bsf(n)+1)) as u64
}
fn bits_reach_start(bits:usize)->u64{
    assert!(bits<=64);
    (1u64<<bits)-1
}
fn bits_end_mask(last_bit:usize,bits:usize)->u64{
    assert!(bits<=last_bit&&last_bit<=64);
    ((1u64<<bits)-1)<<(last_bit-bits)
}