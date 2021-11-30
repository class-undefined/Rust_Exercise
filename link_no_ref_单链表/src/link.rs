type LinkNodeOption<T> = Option<Box<LinkNode<T>>>;
pub struct LinkNode<T> {
    val: T,
    next: LinkNodeOption<T>
}
impl <T>LinkNode<T> {
    pub fn new(val: T) -> Self {
        Self {val, next: None}
    }

    pub fn from(val: T, next: LinkNodeOption<T>) -> LinkNodeOption<T> {
        let node = Self {val, next};
        node.to_option()
    }

    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn to_option(self) -> LinkNodeOption<T> {
        Some(self.to_box())
    }
}

pub struct LinkList<T> {
    head: LinkNodeOption<T>,
    size: usize
}

impl <T>LinkList<T> {
    pub fn insert(&self, begin: usize, val: T) -> () {
        /* 不合法的begin数值触发panic */
        if begin < 0 || begin > self.size {
            panic!("非法的索引begin: {}", begin);
        }
        /* 生成插入的节点 */
        let node = LinkNode::new(val).to_option();
        
        /* 如果begin为0，初始化头指针 */
        for _i in 0..self.size - 1 {
            
        }
        /* 找到需要插入的位置begin所在的节点, */
    }
}