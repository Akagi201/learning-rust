struct Student {
    id: u32,
    name: String,
    age: u32,
    grade: String,
}

struct StudentManagementSystem {
    students: Vec<Student>,
}

impl StudentManagementSystem {
    fn new() -> Self {
        StudentManagementSystem {
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn remove_student(&mut self, id: u32) {
        self.students.retain(|student| student.id != id);
    }

    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|student| student.id == id)
    }

    fn update_student_grade(&mut self, id: u32, new_grade: String) {
        if let Some(student) = self.students.iter_mut().find(|student| student.id == id) {
            student.grade = new_grade;
        }
    }

    fn print_all_students(&self) {
        for student in &self.students {
            println!("ID: {}\nName: {}\nAge: {}\nGrade: {}\n", student.id, student.name, student.age, student.grade);
        }
    }
}

fn main() {
    let mut system = StudentManagementSystem::new();

    // 添加学生
    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        age: 18,
        grade: String::from("A"),
    };
    system.add_student(student1);

    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        age: 17,
        grade: String::from("B"),
    };
    system.add_student(student2);

    // 打印所有学生信息
    system.print_all_students();

    // 更新学生成绩
    system.update_student_grade(1, String::from("A+"));

    // 打印更新后的学生信息
    if let Some(student) = system.get_student(1) {
        println!("Updated grade for {}: {}", student.name, student.grade);
    }

    // 移除学生
    system.remove_student(2);

    // 再次打印所有学生信息
    system.print_all_students();
}