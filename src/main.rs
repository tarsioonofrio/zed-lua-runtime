use mlua::Lua;

fn main() -> mlua::Result<()> {
    // Cria um novo ambiente Lua
    let lua = Lua::new();

    // Script Lua como string
    let script = r#"
        function add(a, b)
            return a + b
        end

        return add(3, 4)
    "#;

    // Executa o script e pega o retorno
    let result: i32 = lua.load(script).eval()?;
    println!("Resultado: {}", result);

    Ok(())
}
