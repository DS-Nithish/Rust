def calculate_area():
    name = input("Enter a shape (rectangle,triangle,square,circle,parallelogram)").lower()

    if name == 'rectangle':
        l = int(input("Enter the length of the rectangle:"))
        b = int(input("Enter the breadth of the rectangle:"))
        rect_area = l * b
        print(f"The area of rectangle is {rect_area}")

    elif name == 'square':
        s = int(input("Enter the sides of square:"))
        square_area = s*s
        print(f"The area of square is {square_area}.")
    
    elif name == 'triangle':
        b = float(input("Enter the breadth of triangle:"))
        h = float(input("Enter the height of triangle:"))
        tri_area = 0.5*b*h
        print(f"The are of triangle is {tri_area}")
    
    elif name == 'parallelogram':
        b = float(input("Enter the breadth of triangle:"))
        h = float(input("Enter the height of triangle:"))
        para_area = b*h
        print(f"The are of triangle is {para_area}")

    elif name == 'circle':
        r = float(input("Enter the radius of circle:"))
        pi = 3.14
        circ_area = pi*r*r
        print(f"The area of circle is {circ_area}")
    
    else:
        print("The shape is not available")

if __name__ == "__main__":
    calculate_area()