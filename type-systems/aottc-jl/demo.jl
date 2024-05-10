function calc_pot_temp(t::Float64, p::Float64)::Float64 
    return t * (1000.0/p)^0.286
end

function demo()
    println(calc_pot_temp(273.0, "1000.0"))
end

demo()
