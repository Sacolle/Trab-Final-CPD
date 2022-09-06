use crate::io::parser::Player;

//uso da implementação do quicksort usando a partição de lomuto com o pivô ao fim
pub fn quicksort<T: PartialOrd + Clone>(vec:&mut Vec<T>){
	let last = vec.len() - 1;
	sort(vec.as_mut_slice(), 0, last);
}

fn sort<T: PartialOrd + Clone>(vec: &mut [T],i:usize,f:usize){
	if f > i {
		let p = lomuto(vec,i,f);
		if p != 0{
			sort(vec, i, p - 1);
		}
		sort(vec, p + 1, f);
	}
}

fn lomuto<T: PartialOrd + Clone>(vec: &mut [T],i:usize,f:usize)->usize{
	let x = vec[f].clone();
	let mut j = i;
	
	for k in i..f{
		if vec[k] <= x {
			vec.swap(j, k);
			j += 1;
		}
	}
	vec[f] = vec[j].clone();
	vec[j] = x;
	j
}

//implementação de ordenamento ao player
impl PartialOrd for Player{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.rating <= other.rating{
			Some(std::cmp::Ordering::Greater)
		}else{
			Some(std::cmp::Ordering::Less)
		}
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_quick_s(){
		let mut nums:Vec<usize> = (1..=100).rev().collect();
		quicksort(&mut nums);
		
		assert!((1..=100).zip(nums).all(|val|val.0 == val.1))
	}
	#[test]
	fn float_quick(){
		let mut nums = vec![0.3420, 2.2131, 4.3093, 3.1912, 4.4645,
			1.6212, 4.0069, 3.3633, 1.3795, 4.7645, 4.5465, 2.3999, 4.4801,
			1.9577, 1.2240, 3.6727, 0.1959, 1.4974, 0.4841, 3.2589, 4.3013,
			1.3970, 0.4004, 4.7214, 4.5112, 4.1299, 1.5965, 4.7096, 2.9703,
			0.1446, 1.9134, 3.1050, 4.0071, 0.4361, 3.8416, 4.3023, 2.9888,
			1.2042, 3.8167, 2.6749, 2.1282, 3.3837, 0.4086, 3.2008, 3.0737,
			2.7977, 4.0385, 3.3336, 0.3347, 1.9769];
	
		let mut nums2 = nums.clone();
		nums2.as_mut_slice().sort_by(|a, b| a.partial_cmp(b).unwrap());
		
		quicksort(&mut nums);

		assert_eq!(nums,nums2);
	}

}