/// Lista estática das top‑1000 portas (mesma lista do ArchScan original em Python)
/// e funções utilitárias de parsing de faixas de portas.

pub const TOP_1000_PORTS: &[u16] = &[
    1, 2, 3, 7, 9, 13, 17, 19, 20, 21, 22, 23, 25, 26, 37, 38, 42, 49, 53,
    67, 68, 69, 79, 80, 81, 82, 88, 100, 106, 110, 111, 112, 113, 119, 120,
    123, 135, 136, 137, 138, 139, 143, 144, 158, 161, 162, 177, 179, 192, 199,
    207, 217, 254, 255, 280, 311, 363, 389, 402, 407, 427, 434, 443, 444, 445,
    464, 465, 497, 500, 502, 512, 513, 514, 515, 517, 518, 520, 539, 543, 544,
    548, 554, 559, 587, 593, 623, 625, 626, 631, 636, 639, 643, 646, 657, 664,
    682, 683, 684, 685, 686, 687, 688, 689, 764, 767, 772, 773, 774, 775, 776,
    780, 781, 782, 786, 787, 789, 800, 808, 814, 826, 829, 838, 873, 902, 903,
    944, 959, 965, 983, 989, 990, 993, 995, 996, 997, 998, 999, 1000, 1001,
    1007, 1008, 1012, 1013, 1014, 1019, 1020, 1021, 1022, 1023, 1024, 1025,
    1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037,
    1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045, 1046, 1047, 1048, 1049,
    1050, 1051, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060, 1064, 1065,
    1066, 1067, 1068, 1069, 1070, 1071, 1072, 1080, 1081, 1087, 1088, 1090,
    1100, 1101, 1105, 1110, 1124, 1200, 1214, 1234, 1346, 1419, 1433, 1434,
    1455, 1457, 1484, 1485, 1521, 1524, 1645, 1646, 1701, 1718, 1719, 1720,
    1723, 1755, 1761, 1782, 1801, 1804, 1812, 1813, 1885, 1886, 1900, 1901,
    1993, 1998, 2000, 2001, 2002, 2005, 2048, 2049, 2051, 2103, 2105, 2107,
    2121, 2148, 2160, 2161, 2222, 2223, 2343, 2345, 2362, 2383, 2401, 2601,
    2717, 2869, 2967, 3000, 3001, 3052, 3128, 3130, 3283, 3296, 3306, 3343,
    3389, 3401, 3456, 3457, 3659, 3664, 3689, 3690, 3702, 3703, 3986, 4000,
    4001, 4008, 4045, 4444, 4500, 4666, 4672, 4899, 5000, 5001, 5002, 5003,
    5009, 5010, 5050, 5051, 5060, 5093, 5101, 5120, 5190, 5351, 5353, 5355,
    5357, 5432, 5500, 5555, 5631, 5632, 5666, 5800, 5900, 5901, 6000, 6001,
    6002, 6004, 6050, 6112, 6346, 6347, 6646, 6970, 6971, 7000, 7070, 7937,
    7938, 8000, 8001, 8008, 8009, 8010, 8031, 8080, 8081, 8181, 8193, 8443,
    8888, 8900, 9000, 9001, 9020, 9090, 9100, 9102, 9103, 9199, 9200, 9370,
    9876, 9877, 9950, 9999, 10000, 10010, 10080,
];

/// Interpreta uma string de portas nos formatos:
/// - Range:   `"0-1024"`
/// - Lista:   `"22,80,443"`
/// - Única:   `"80"`
pub fn parse_ports(s: &str) -> Result<Vec<u16>, String> {
    let s = s.trim();

    if s.contains('-') {
        // Range de portas
        let parts: Vec<&str> = s.splitn(2, '-').collect();
        if parts.len() != 2 {
            return Err("Formato de range inválido. Use: 0-1024".to_string());
        }
        let start: u16 = parts[0]
            .trim()
            .parse()
            .map_err(|_| format!("Porta inicial inválida: '{}'", parts[0].trim()))?;
        let end: u16 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("Porta final inválida: '{}'", parts[1].trim()))?;

        if start > end {
            return Err(format!(
                "Porta inicial ({}) maior que a final ({}).",
                start, end
            ));
        }
        Ok((start..=end).collect())
    } else if s.contains(',') {
        // Lista de portas
        s.split(',')
            .map(|p| {
                p.trim()
                    .parse::<u16>()
                    .map_err(|_| format!("Porta inválida na lista: '{}'", p.trim()))
            })
            .collect()
    } else {
        // Porta única
        let port: u16 = s
            .parse()
            .map_err(|_| format!("Porta inválida: '{}'", s))?;
        Ok(vec![port])
    }
}

// ─── Testes ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let ports = parse_ports("0-100").unwrap();
        assert_eq!(ports.len(), 101);
        assert_eq!(ports[0], 0);
        assert_eq!(ports[100], 100);
    }

    #[test]
    fn test_lista() {
        let ports = parse_ports("22,80,443").unwrap();
        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn test_porta_unica() {
        let ports = parse_ports("443").unwrap();
        assert_eq!(ports, vec![443]);
    }

    #[test]
    fn test_porta_invalida() {
        assert!(parse_ports("abc").is_err());
    }

    #[test]
    fn test_range_invertido() {
        assert!(parse_ports("1024-0").is_err());
    }

    #[test]
    fn test_porta_com_espacos() {
        let ports = parse_ports("  80  ").unwrap();
        assert_eq!(ports, vec![80]);
    }
}
