use std::{str::Chars, iter::Peekable};


struct Trie<K>
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
				if ch > branch.ch{
					if branch.right.is_none(){
						if chs.next().is_none(){  //move o iterador
							Trie::_insert_ch_with_key(&mut branch.right, ch, value)
						}else{
							Trie::_insert_ch(&mut branch.right, ch);
						}
					}
					head = branch.right.as_deref_mut();
				} 
				else if ch < branch.ch{
					if branch.left.is_none(){
						if chs.next().is_none(){  //move o iterador
							Trie::_insert_ch_with_key(&mut branch.left, ch, value)
						}else{
							Trie::_insert_ch(&mut branch.left, ch);
						}
					}
					head = branch.left.as_deref_mut();
				}
				else{
					if branch.mid.is_none(){
						if chs.peek().is_none(){
							Trie::_insert_ch_with_key(&mut branch.left, ch, value)
						}else{
							Trie::_insert_ch(&mut branch.mid, ch);
						}
					}
					chs.next();
					head = branch.mid.as_deref_mut();
				}
			}else{
				println!("nunca vai chegar aqui");
				break;
			}
		} 
	}
	fn _insert_ch(head:&mut Branch<K>,ch:char){
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
				println!("nunca chega aqui");
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
			if let Some(val) = &head.value{
				res.push(val.clone());
			}
			Trie::_get_prefix(&mut res,&head.left);
			Trie::_get_prefix(&mut res,&head.mid);
			Trie::_get_prefix(&mut res,&head.right);
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
}