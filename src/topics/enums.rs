enum Disk_type {
    SSD,
    HDD,
}
enum Disk_Size {
    GB(u32),
    TB(u32),
}
struct Disk {
    disk_type: Disk_type,
    disk_size: Disk_Size,
}
fn main() {
    let disk = Disk {
        disk_type: Disk_type::SSD,
        disk_size: Disk_Size::TB(1),
    };
    match disk.disk_size {
        Disk_Size::GB(size) => println!("Disk size is {} GB", size),
        Disk_Size::TB(size) => println!("Disk size is {} TB", size),
    }
}
