/**
 * https://www.reddit.com/r/dailyprogrammer/comments/bqy1cf/20190520_challenge_378_easy_the_havelhakimi/
 */

use rand::Rng;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Eq, Hash)]
struct Person {
    ind: u64,
}
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.ind == other.ind
    }
}

/*
struct Relationship {
    a: Rc<Person>,
    b: Rc<Person>,
}
*/

fn main() {
    // Setup problem graph:
    // TODO(lucasw) create a random graph of around 10 people,
    // with random bidirectional links between them,
    // e.g. the probability any 2 people know each other could
    // be the same for all cases.
    // Then output the number of links for each person, sorted
    let mut people : Vec<Rc<Person>> = Vec::new();
    let num_people = 10;
    for ind in 1..=num_people {
        let person: Rc<Person> = Rc::new(
            Person{ind: ind}
        );
        // println!("person {}", person.ind);
        people.push(person);
    }

    // TODO(lucasw) could use the reference count in the Rc
    let mut counts = HashMap::new();
    print!("     ");
    for person in people.iter() {
        print!(" {:02}  ", person.ind);
    //     counts.insert(person, 0);
    }
    print!("\n");

    let mut rng = rand::thread_rng();
    let probability = 0.3;
    // let mut relationships : Vec<Relationship> = Vec::new();
    for person_a in people.iter() {
        print!(" {:02} ", person_a.ind);
        for person_b in people.iter() {
            // if Rc::ptr_eq(person_a, person_b) {
            // TODO(lucasw) how to modify the iter to skip these to begin with
            if person_b.ind >= person_a.ind {
                // println!("person {} {}", person_a.ind, person_b.ind);
                continue;
            }
            if rng.gen::<f64>() < probability {
                // let relationship = Relationship{
                //     a: Rc::clone(&person_a),
                //     b: Rc::clone(&person_b),
                // };
                // relationships.push(relationship);
                // println!("person {} knows {}", person_a.ind, person_b.ind);
                print!("  1  ");
                *counts.entry(person_a).or_insert(0) += 1;
                *counts.entry(person_b).or_insert(0) += 1;
            } else {
                print!("  -  ");
            }
        }
        print!("\n");
    }

    // now output number of known people
    for person in people.iter() {
        println!("person {:02} knows {} people", person.ind, counts.entry(person).or_insert(0));
    }
}
