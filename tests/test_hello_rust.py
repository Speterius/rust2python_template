import hello_rust


def test_add():
	assert hello_rust.add(3, 4) == 7
	assert hello_rust.add(3, -4) == -1
	assert hello_rust.add(123, 123) == 246
	assert hello_rust.add(-123, -123) == -246


def test_classes():
	rect = hello_rust.Rectangle(2, 3)
	assert rect.area() == 6
