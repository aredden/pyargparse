from rust_arg_parser import parse_command

parse_command

boolean_flags = {"use-emoji"}
command = "noodle is named noodle --name John --age 30.4 --scores "\
    "[I love to eat cake, lemon cakes, peanut butter] "\
    "--prompt I love to eat pancakes! also- I like to eat waffles! "\
    "--use-emoji --nums [1,2,3,4,5,6,7,  8,9,10]"
parsed = parse_command(command, boolean_flags)

print(parsed)
