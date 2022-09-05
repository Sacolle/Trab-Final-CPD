use std::{str::Chars, iter::Peekable};


pub struct Trie<K>
where K: Clone 
{
	left:  Branch<K>,
	mid:   Branch<K>,
	right: Branch<K>,
	ch:    char,
	value: Option<K>
}
type Branch<K> = Option<Box<Trie<K>>>;

impl<K> Trie<K>
where K: Clone 
{
	pub fn init()->Self{
		let trie:Trie<K> = Trie{
			left:None,
			mid:None,
			right:None,
			ch: ' ',
			value:None
		};
		trie
	}

	pub fn insert_str(&mut self, word:&str, value: &K){
		if word.is_empty(){return;}
		let mut chs = word.chars().peekable();
		let insert_node = Trie::mut_travel(self, &mut chs);
		Trie::_insert_str(insert_node, &mut chs, value);
	}

	fn _insert_str(mut head: &mut Trie<K>,chs:&mut Peekable<Chars>,value: &K){
		//inserindo uma a parte de uma palavra que já existia
		if chs.peek().is_none(){
			head.value = Some(value.clone());
			return;
		}
		
		let f_ch = chs.next().unwrap();

		//a função mut_travel retorna o nó anterior ao fim,
		//nesse caso caso tente-se inserir uma palavra q já exista, um nó adicional ao fim seria colocado.
		if chs.peek().is_none() && head.value.is_some(){
			return;
		}

		if f_ch > head.ch{
			//right
			if chs.peek().is_none(){
				Trie::_insert_ch_with_key(&mut head.right, f_ch, value);
			}else{
				Trie::_insert_ch(&mut head.right, f_ch);
			}
			head = head.right.as_deref_mut().unwrap();
		}else if f_ch < head.ch{
			//left
			if chs.peek().is_none(){
				Trie::_insert_ch_with_key(&mut head.left, f_ch, value);
			}else{
				Trie::_insert_ch(&mut head.left, f_ch);
			}
			head = head.left.as_deref_mut().unwrap();
		}else{
			//mid
			if chs.peek().is_none(){
				Trie::_insert_ch_with_key(&mut head.mid, f_ch, value);
			}else{
				Trie::_insert_ch(&mut head.mid, f_ch);
			}
			head = head.mid.as_deref_mut().unwrap();
		}

		while let Some(ch) = chs.next(){
			if !head.mid.is_none(){
				panic!("erro na função mut_travel ou logcia está errada");
			}
			if chs.peek().is_none(){
				Trie::_insert_ch_with_key(&mut head.mid, ch, value);
			}else{
				Trie::_insert_ch(&mut head.mid, ch);
			}
			head = head.mid.as_deref_mut().unwrap();
		}
	}
	fn _insert_ch(head:&mut Branch<K>,ch:char){
		//println!("insert char: {}",ch);
		//unsafe{C += 1;}
		let next:Trie<K> = Trie{
			left:None,
			mid:None,
			right:None,
			ch,
			value: None
		};
		*head = Some(Box::new(next));
	}
	fn _insert_ch_with_key(head:&mut Branch<K>,ch:char,value:&K){
		//println!("insert char: {} with KEY",ch);
		//unsafe{C += 1;}
		let next:Trie<K> = Trie{
			left:None,
			mid:None,
			right:None,
			ch,
			value: Some(value.clone())
		};
		*head = Some(Box::new(next));
	}

	pub fn mut_travel(&mut self,chs:&mut Peekable<Chars>)->&mut Trie<K>{
		//println!("searching with: {:?}",chs);
		let mut head: &mut Trie<K> = self;
		loop{
			if let Some(ch) = chs.peek(){
				//println!("{} == {}",ch, &head.ch);
				if ch > &head.ch{
					if head.right.is_none(){
						return head;
					}
					head = head.right.as_deref_mut().unwrap();
				} 
				else if ch < &head.ch{
					if head.left.is_none(){
						return head;
					}
					head = head.left.as_deref_mut().unwrap();
				}
				else{
					if head.mid.is_none(){
						return head;
					}
					chs.next();
					if chs.peek().is_none(){ //chegou-se no fim da palavra
						return head;
					}
					head = head.mid.as_deref_mut().unwrap();
				}
			}else{
				println!("should not get here teoreticaly");
				return head;
			}
		}
	}

	pub fn peek_travel(&self,chs:&mut Peekable<Chars>)->&Branch<K>{
		//println!("searching with: {:?}",chs);
		let first_ch = chs.peek().unwrap();
		let mut head = if *first_ch > self.ch {
			&self.right
		}else if *first_ch < self.ch {
			&self.left
		}else {
			chs.next();
			&self.mid
		};
		while let Some(branch) = head{
			if let Some(ch) = chs.peek(){
				//println!("{} == {}",ch, &branch.ch);
				if ch > &branch.ch{
					head = &branch.right;
				} 
				else if ch < &branch.ch{
					head = &branch.left;
				}
				else{
					chs.next();
					if chs.peek().is_none(){ //chegou-se no fim da palavra
						break; 
					}
					head = &branch.mid;
				}
			}else{
				break;
			}
		}
		head //chegou-se no fim da corrente
	}
	#[allow(dead_code)]
	pub fn get(&self,word:&str)->Option<&K>{
		let mut chs = word.chars().peekable();
		if let Some(target) = Trie::peek_travel(self,&mut chs){
			if chs.peek().is_none(){
				return target.value.as_ref();
			}
			return None;
		}
		None
	}
	pub fn get_prefix(&self,word:&str)->Vec<K>{
		let mut res:Vec<K> = Vec::new();
		let mut chs = word.chars().peekable();
		if let Some(head) = Trie::peek_travel(&self, &mut chs){
			if let Some(mid_head) = &head.mid{
				if let Some(val) = &head.value{
					res.push(val.clone());
				}
				Trie::_get_prefix(&mut res,&mid_head.left);
				Trie::_get_prefix(&mut res,&mid_head.mid);
				Trie::_get_prefix(&mut res,&mid_head.right);
			}
		}
		res
	}
	fn _get_prefix(res:&mut Vec<K>,branch:&Branch<K>){
		if let Some(head) = branch{
			if let Some(val) = &head.value{
				res.push(val.clone());
			}
			Trie::_get_prefix(res,&head.left);
			Trie::_get_prefix(res,&head.mid);
			Trie::_get_prefix(res,&head.right);
		}
	}
	fn _drop(branch: &mut Branch<K>){
		if let Some(mut head) = branch.take(){
			Trie::_drop(&mut head.left);
			Trie::_drop(&mut head.mid);
			Trie::_drop(&mut head.right);
		}
	}
}


impl<K> Drop for Trie<K>
where K: Clone
{
    fn drop(&mut self){
		Trie::_drop(&mut self.left);
		Trie::_drop(&mut self.mid);
		Trie::_drop(&mut self.right);
    }
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn init_trie(){
		let v1 = (String::from("carro"),1);
		let mut trie:Trie<i32> = Trie::init();
		trie.insert_str(&v1.0,& v1.1);

		let mut trie:Branch<i32> = trie.right.take();

		let mut word = "carro".chars();

		while let Some(mut mid) = trie{
			let ch = word.next().unwrap();
			println!("{} == {}",mid.ch,ch);
			assert_eq!(mid.ch,ch);
			trie = mid.mid.take();
		}
	}
	#[test]
	fn base_trie(){
		let v1 = (String::from("carro"),1);
		let v2 = (String::from("carinha"),2);
		let v3 = (String::from("cramunhão"),3);
		let v4 = (String::from("erro"),4);
		let v5 = (String::from("abaco"),5);

		let mut trie = Trie::init();
		trie.insert_str(&v1.0, &v1.1);
		trie.insert_str(&v2.0, &v2.1);
		trie.insert_str(&v3.0, &v3.1);
		trie.insert_str(&v4.0, &v4.1);
		trie.insert_str(&v5.0, &v5.1);

		let res = trie.get("carro");
		assert_eq!(res, Some(&1));
	
		let res = trie.get("carinha");
		assert_eq!(res, Some(&2));
	
		let res = trie.get("cari");
		assert_eq!(res, None);

		//println!("char da direita: {}",trie.right.as_ref().unwrap().ch);
		let res = trie.get("erro");
		assert_eq!(res, Some(&4));

		let res = trie.get("abaco");
		assert_eq!(res, Some(&5));
	}
	#[test]
	fn prefix_fetch(){
		let v1 = (String::from("carro"),1);
		let v2 = (String::from("carinha"),2);
		let v3 = (String::from("cramunhão"),3);
		let v4 = (String::from("capitão"),4);

		let mut trie = Trie::init();
		trie.insert_str(&v1.0, &v1.1);
		trie.insert_str(&v2.0, &v2.1);
		trie.insert_str(&v3.0, &v3.1);
		trie.insert_str(&v4.0, &v4.1);


		let res = trie.get_prefix("ca");
		let target = vec![1,2,4];
		if !target.iter().all(|x|res.contains(x)){
			panic!("vec val: {:?}",res);
		}
	}
}