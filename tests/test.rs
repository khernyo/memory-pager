extern crate memory_pager;

use memory_pager::Pager;

#[test]
fn can_create_default() {
  let pager = Pager::default();
  assert_eq!(pager.len(), 0);
}

#[test]
fn can_create_with_size() {
  let pager = Pager::new(1024);
  assert_eq!(pager.len(), 0);
}

#[test]
fn can_get() {
  let mut pager = Pager::default();
  {
    let page = pager.get_mut_or_alloc(0);
    assert_eq!(page.len(), 1024);
  }

  {
    let page = pager.get_mut_or_alloc(3);
    assert_eq!(page.len(), 1024);
  }
  assert_eq!(pager.len(), 4);
}

#[test]
fn can_alloc() {
  let mut pager = Pager::default();
  {
    let page = pager.get_mut_or_alloc(16);
    assert_eq!(page.len(), 1024);
  }

  {
    let page = pager.get_mut_or_alloc(32);
    assert_eq!(page.len(), 1024);
  }
}

#[test]
fn can_write() {
  let mut pager = Pager::default();
  let page = pager.get_mut_or_alloc(0);
  page[0] = 1;

  assert_eq!(1, page[0]);
  assert_eq!(0, page[1]);
}

#[test]
fn can_check_offset() {
  let mut pager = Pager::default();
  let page = pager.get_mut_or_alloc(1);

  assert_eq!(1024, page.offset());
}

#[test]
fn can_access_nodes() {
  let pager = &mut Pager::default();
  assert!(pager.get(0).is_none());
  pager.get_mut_or_alloc(0);
  assert!(pager.get(0).is_some());
}

#[test]
fn can_get_31() {
  let pager = &mut Pager::default();
  pager.get_mut_or_alloc(31);
  assert!(pager.get(31).is_some());
}

#[test]
fn set_122() {
  let pager = &mut Pager::default();
  pager.get_mut_or_alloc(122);
}

#[test]
fn can_iterate_over_pager() {
  let mut pager = Pager::default();
  pager.get_mut_or_alloc(0);
  pager.get_mut_or_alloc(2);

  for (i, page) in pager.iter().enumerate() {
    match i {
      0 => assert!(page.is_some(), "0 is some"),
      1 => assert!(page.is_none(), "1 is none"),
      2 => assert!(page.is_some(), "2 is some"),
      i => panic!("Index {} out of bounds", i),
    }
  }
}

#[test]
fn can_iterate_over_pages() {
  let mut pager = Pager::default();
  pager.get_mut_or_alloc(0);

  for page in pager.iter() {
    match page {
      None => panic!("Index 0 should contain a page"),
      Some(page) => {
        for num in page.iter() {
          assert_eq!(*num, 0);
        }
      }
    }
  }
}
