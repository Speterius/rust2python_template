import hello_rust


def test_hello_rust():
	assert hello_rust.add(3, 4) == 7
	assert hello_rust.add(3, -4) == -1
	assert hello_rust.add(123, 123) == 246
	assert hello_rust.add(-123, -123) == -246