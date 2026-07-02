// TODO: Sistema de renderização individual de tiles por sprite.
//
// Atualmente cada tile do mundo gera 1 entidade Sprite no setup_map_view.
// Para mapas de 275×275 isso resulta em ~75.625 entidades, causando lag
// em zoom máximo (ver metrics.rs).
//
// Próxima etapa: substituir sprites individuais por chunks visuais:
// - 1 chunk = mesh consolidada de N×N tiles (sugestão: 32×32)
// - Renderizar apenas chunks dentro do frustum da câmera
// - Marcar chunks dirty quando tiles adjacentes mudam
//
// Este arquivo será o local dos sistemas de render dos tiles quando
// a transição para chunks for implementada.
