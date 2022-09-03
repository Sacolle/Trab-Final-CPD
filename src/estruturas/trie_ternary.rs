use std::{str::Chars, iter::Peekable};


struct Trie<K>
where K: Clone 
{
	left:  Branch<K>,
	mid:   Branch<K>,
	right: Branch<K>,
	pub ch:    char,
	value: Option<K>
}
type Branch<K> = Option<Box<Trie<K>>>;


impl<K> Trie<K>
where K: Clone 
{
	pub fn init(seed:&str,value: K)->Self{
		assert!(!seed.is_empty());
		let mut chs = seed.chars().peekable();
		let mut trie:Trie<K> = Trie{
			left:None,
			mid:None,
			right:None,
			ch: chs.next().unwrap(),
			value:None
		};
		println!("Chars {:?}",chs);
		Trie::_insert_str(Some(&mut trie),&mut chs, &value);
		trie
	}
	pub fn insert_str(&mut self, word:&str, value: &K){
		let mut chs = word.chars().peekable();
		if chs.peek().is_none(){
			return;
		}
		let first_ch = chs.next().unwrap();
		let head = if first_ch > self.ch {
			self.right.as_deref_mut()
		}else if first_ch < self.ch {
			self.left.as_deref_mut()
		}else {
			self.mid.as_deref_mut()
		};
		Trie::_insert_str(head, &mut chs, value);
	}

	fn _insert_str(mut head:Option<&mut Trie<K>>,chs:&mut Peekable<Chars>,value: &K){
		while let Some(branch) = head{
			if let Some(_ch) = chs.peek(){
				let ch = *_ch;
				chs.next();
				if branch.mid.is_none(){
					if chs.peek().is_none(){
						Trie::_insert_ch_with_key(&mut branch.mid, ch, value);
					}else{
						Trie::_insert_ch(&mut branch.mid, ch);
					}
					head = branch.mid.as_deref_mut();
				}else{
					if ch > branch.ch{
						if branch.right.is_none(){
							if chs.peek().is_none(){  //move o iterador
								Trie::_insert_ch_with_key(&mut branch.right, ch, value)
							}else{
								Trie::_insert_ch(&mut branch.right, ch);
							}
						}
						head = branch.right.as_deref_mut();
					} 
					else if ch < branch.ch{
						if branch.left.is_none(){
							if chs.peek().is_none(){  //move o iterador
								Trie::_insert_ch_with_key(&mut branch.left, ch, value);
							}else{
								Trie::_insert_ch(&mut branch.left, ch);
							}
						}
						head = branch.left.as_deref_mut();
					}else{
						head = branch.mid.as_deref_mut();
					}
				}
			}else{
				println!("nunca vai chegar aqui {}",branch.ch);
				break;
			}
		}
	}
	fn _insert_ch(head:&mut Branch<K>,ch:char){
		//println!("inserted {}",ch);
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
		//println!("inserted {} and key",ch);
		let next:Trie<K> = Trie{
			left:None,
			mid:None,
			right:None,
			ch,
			value: Some(value.clone())
		};
		*head = Some(Box::new(next));
	}
	fn peek_travel(&self,chs:&mut Peekable<Chars>)->&Branch<K>{
		let first_ch = chs.next().unwrap();

		let mut head = if first_ch > self.ch {
			&self.right
		}else if first_ch < self.ch {
			&self.left
		}else {
			&self.mid
		};
		while let Some(branch) = head{
			if let Some(ch) = chs.peek(){
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
	fn base_trie(){
		let v1 = (String::from("carro"),1);
		let v2 = (String::from("carinha"),2);
		let v3 = (String::from("cramunhão"),3);

		let mut trie = Trie::init(&v1.0, v1.1);
		trie.insert_str(&v2.0, &v2.1);
		trie.insert_str(&v3.0, &v3.1);
		let res = trie.get("carro");
		assert_eq!(res, Some(&1));
		let res = trie.get("carinha");
		assert_eq!(res, Some(&2));
		let res = trie.get("cari");
		assert_eq!(res, None);
	}
	#[test]
	fn prefix_fetch(){
		let v1 = (String::from("carro"),1);
		let v2 = (String::from("carinha"),2);
		let v3 = (String::from("cramunhão"),3);

		let mut trie = Trie::init(&v1.0, v1.1);
		trie.insert_str(&v2.0, &v2.1);
		trie.insert_str(&v3.0, &v3.1);


		let res = trie.get_prefix("ca");
		assert_eq!(res, vec![2,1]);
	}
}