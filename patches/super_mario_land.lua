
function Set (list)
    local set = {}
    for _, l in ipairs(list) do set[l] = true end
    return set
end

enemy_types = {
    0x00, 0x02, 0x03, 0x04, 0x06, 0x08, 0x09, 0x0C,
    0x0E, 0x10, 0x16, 0x1A, 0x1D, 0x24, 0x25, 0x2F,
    0x31, 0x32, 0x35, 0x3C, 0x3F, 0x42, 0x47, 0x48,
    0x49, 0x52, 0x53, 0x54, 0x55, 0x56, 0x61,
}

important_types = Set {
    0x03, 0x0A, 0x0B,
    0x36, 0x38, 0x39,
    0x3A, 0x3B, 0x47,
}

function randomize_enemies(start_addr, end_addr)
    for addr = start_addr, end_addr, 3 do
        enemy_type_addr = addr + 2
        enemy_type = rom:read_byte(enemy_type_addr)

        if important_types[enemy_type] ~= true then
            rom:write_byte(enemy_type_addr, rng:choose(enemy_types))
        end
    end
end

randomize_enemies(0xA002, 0xA070) -- 1-1
randomize_enemies(0xA073, 0xA0FC) -- 1-2
randomize_enemies(0xA0FE, 0xA18D) -- 1-3
randomize_enemies(0x5179, 0x5220) -- 2-1
randomize_enemies(0x5222, 0x5299) -- 2-2
randomize_enemies(0x529B, 0x530F) -- 2-3
randomize_enemies(0xCE74, 0xCF1B) -- 3-1
randomize_enemies(0xCF1D, 0xCFD6) -- 3-2
randomize_enemies(0xCFD8, 0xD03D) -- 3-3
randomize_enemies(0x5311, 0x5403) -- 4-1
randomize_enemies(0x5405, 0x54D3) -- 4-2
randomize_enemies(0x54D5, 0x55B8) -- 4-3
