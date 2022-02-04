use cw1_2::AnimalTracker;

fn main() {
    let mut t = AnimalTracker::new("Rob".to_string(), 0.0, 0.0);

    println!("Animal: {}", t.name);
    println!("Location: ({},{})", t.location.get_x(), t.location.get_y());
    println!("Distance travelled: {}", t.get_distance_travelled());
    println!("Should be: 0 \n");

    t.move_(1.0, 2.0);

    println!("Animal: {}", t.name);
    println!("Location: ({},{})", t.location.get_x(), t.location.get_y());
    println!("Distance travelled: {}", t.get_distance_travelled());
    println!("Should be: 2.23606797749979 \n");

    t.move_(3.0, 5.0);

    println!("Animal: {}", t.name);
    println!("Location: ({},{})", t.location.get_x(), t.location.get_y());
    println!("Distance travelled: {}", t.get_distance_travelled());
    println!("Should be: 5.841619252963779 or 5.842 to 3.d.p \n");
}

// /**
//  *
//  * This is the CIS1112 Coursework 1.2 assignment helper file.
//  *
//  *
//  * File name: CW1_2.java
//  * Package: default
//  * Created:	January 9th, 2020
//  * @Author:	Dr. Robert Lyon (lyonro@edgehill.ac.uk)
//  *
//  * Contact:	lyonro@edgehill.ac.uk
//  * Web:		https://www.edgehill.ac.uk/computerscience/people/academic-staff/robert-lyon/
//  *
//  */
// /**
//  * This is the coursework helper file. Do not,
//  *
//  * i) rename the file.
//  * ii) edit the file.
//  *
//  * @author Dr. Robert Lyon
//  *
//  */
// public class CW1_2
// {
// 	/**
// 	 * Main entry point to the application.
// 	 *
// 	 * @param args unused command line arguments.
// 	 */
// 	public static void main(String[] args)
// 	{
// 		// Create the animal tracker.
// 		AnimalTracker t = new AnimalTracker("Rob",0.0,0.0);

// 		print("Animal: " + t.name);
// 		print("Location: (" + t.location.getX() + "," + t.location.getY()+ ")");
// 		print("Distance travelled: " + t.getDistanceTravelled());
// 		print("Should be: 0 \n");

// 		t.move(1.0, 2.0);

// 		print("Animal: " + t.name);
// 		print("Location: (" + t.location.getX() + "," + t.location.getY()+ ")");
// 		print("Distance travelled: " + t.getDistanceTravelled());
// 		print("Should be: 2.23606797749979 \n");

// 		t.move(3.0, 5.0);

// 		print("Animal: " + t.name);
// 		print("Location: (" + t.location.getX() + "," + t.location.getY()+ ")");
// 		print("Distance travelled: " + t.getDistanceTravelled());
// 		print("Should be: 5.841619252963779 or 5.842 to 3.d.p \n");

// 	}

// 	/**
// 	 * I've wrapped the print method, to make my examples clearer.
// 	 *
// 	 * @param text the text to print to standard output.
// 	 */
// 	private static void print(String text)
// 	{
// 		System.out.println(text);
// 	}
// }
