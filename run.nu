ls | where type == dir | each { |it| cd $it.name; cargo run; cd .. }
