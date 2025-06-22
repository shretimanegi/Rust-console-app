use std::fs::File;
use printpdf::*;
use std::io::BufWriter;
use std::io::{self};

struct Student{
    name: String,
    total_marks: f32,
    no_of_subject: u32,
}

impl Student{
    fn average(&self)->f32{
        self.total_marks/self.no_of_subject as f32
    }

    fn grade(&self)->char{
        let avg=self.average();
        if avg>=90.0{
            'A'
        }
        else if avg>=80.0 {
            'B'
        }
        else if avg>=70.0 {
            'C'
        }
        else{
            'D'
        }
    }
}

fn generate_pdf(student:&Student){
    let (doc,page1,layer1)=PdfDocument::new("Student Performance Summary",Mm(210.0),Mm(297.0),"Layer1");
    let current_layer=doc.get_page(page1).get_layer(layer1);
    let font=doc.add_builtin_font(BuiltinFont::Helvetica).expect("Failed to load font");
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).expect("Failed to load bold font");
    let avg=student.average();
    let grade=student.grade();
    current_layer.use_text("STUDENT REPORT CARD", 20.0, Mm(60.0), Mm(270.0), &font_bold);
    use printpdf::{Line, LineCapStyle, LineDashPattern, Point, Color, Rgb};
    let line = Line {
        points: vec![
            (Point::new(Mm(20.0), Mm(265.0)), false),
            (Point::new(Mm(190.0), Mm(265.0)), false),
        ],
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    current_layer.set_outline_thickness(1.0);
    current_layer.set_line_dash_pattern(LineDashPattern::default());
    current_layer.set_line_cap_style(LineCapStyle::Butt);
    current_layer.add_shape(line);

    let content_lines=vec![
        "Student Performance Summary".to_string(),
        "".to_string(),
        format!("Name: {}",student.name),
        format!("Total Marks: {}",student.total_marks),
        format!("Subject: {}",student.no_of_subject),
        format!("Average: {}",avg),
        format!("Grade: {}",grade),
    ];
    
    let mut y=250.0;
    for line in content_lines{
        current_layer.use_text(line,14.0,Mm(20.0),Mm(y), &font);
        y-=10.0;
    }
    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();   
}
fn main(){
    let mut name=String::new();
    let mut total_marks=String::new();
    let mut no_of_subject=String::new();
    println!("Enter student name: ");
    io::stdin().read_line(&mut name).unwrap();
    println!("Enter student total marks: ");
    io::stdin().read_line(&mut total_marks).unwrap();
    println!("Enter student no of subjects: ");
    io::stdin().read_line(&mut no_of_subject).unwrap();
    let student=Student{
        name:name.trim().to_string(),
        total_marks:total_marks.trim().parse().unwrap(),       
        no_of_subject:no_of_subject.trim().parse().unwrap(),      
    };
    generate_pdf(&student);
    println!("PDF has been generated!");
}
