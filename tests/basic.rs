use papyri_lang::errors::PapyriError;

mod common;

assert_ok! {
    hello_world(
        "Hello, world!",
        "<p>Hello, world!</p>",
    );
    
    paragraph_break(
        "Paragraph 1\n\nParagraph 2",
        "<p>Paragraph 1</p><p>Paragraph 2</p>",
    );
    
    simple_function(
        "@b Something",
        "<p><b>Something</b></p>",
    );
    
    positional_argument(
        "@href(`foo.html`) Foo",
        r#"<p><a href="foo.html">Foo</a></p>"#,
    );
    
    named_argument(
        "@image(alt=`Foo`) `foo.png`",
        r#"<img src="foo.png" alt="Foo">"#,
    );
    
    optional_argument(
        "@image `foo.png`",
        r#"<img src="foo.png">"#,
    );
    
    group_function(
        "@b {Something else}",
        "<p><b>Something else</b></p>",
    );
    
    variable(
        "@let(x=5). $x $x",
        "<p>5 5</p>",
    );
    
    line_comment(
        "# Nothing\nSomething",
        "<p>Something</p>",
    );
    
    multiline_comment(
        "<!-- Nothing\nhere -->\nSomething",
        "<p>Something</p>",
    );
    
    line_comment_with_backticks(
        "# ```\nFoobar",
        "<p>Foobar</p>",
    );
    
    multiline_comment_with_escape(
        r"<!-- \-->Foobar",
        "<p>Foobar</p>",
    );
    
    multiline_comment_with_backticks(
        r"<!-- ``` -->Foobar",
        "<p>Foobar</p>",
    );
}

assert_err! {
    verbatim_eof(
        "`some string literal",
        PapyriError::SyntaxError(..),
    );
    
    raise(
        "@raise `foobar`",
        PapyriError::RuntimeError(..),
    );
}