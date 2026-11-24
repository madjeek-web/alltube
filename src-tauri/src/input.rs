// =============================================================================
//  input.rs : vérifier ce que l'utilisateur a tapé dans la barre de recherche
// =============================================================================
//  L'utilisateur peut taper :
//    - un LIEN       (https://www.youtube.com/watch?v=...)  -> on le garde
//    - des MOTS-CLÉS (tove lo habits)                       -> on cherche sur YouTube
//                                                              et on prend le 1er résultat
//  Tout le reste est refusé. Ce fichier est volontairement tout petit et
//  séparé : c'est la première barrière de sécurité, donc on la teste beaucoup
//  (les "tests" en bas du fichier, à lancer avec : cargo test).
// =============================================================================

/// Transforme ce que l'utilisateur a tapé en "cible" pour yt-dlp.
///  - un lien http(s) valide  -> on le garde
///  - sinon des mots-clés      -> "ytsearch1:mots" (= le premier résultat de la recherche)
pub fn normalize_input(raw: &str) -> Result<String, String> {
    let text = raw.trim();
    // Vide, trop long, ou contenant des caractères de contrôle (retours à la ligne, nul...) : refusé.
    if text.is_empty() || text.chars().count() > 2000 || text.chars().any(|c| c.is_control()) {
        return Err("invalid_input".to_string());
    }
    let no_space = !text.contains(char::is_whitespace);
    let looks_like_link = text.starts_with("http://")
        || text.starts_with("https://")
        || (no_space && (text.starts_with("www.") || (text.contains('.') && text.contains('/'))));

    if looks_like_link {
        // "youtube.com/watch?v=x" sans https:// : on l'ajoute nous-mêmes.
        let candidate = if text.contains("://") { text.to_string() } else { format!("https://{text}") };
        let url = url::Url::parse(&candidate).map_err(|_| "invalid_input")?;
        // Seuls http et https sont acceptés (pas file://, pas ftp://...).
        if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
            return Err("invalid_input".to_string());
        }
        Ok(url.to_string())
    } else {
        // Même "-o malin" reste un simple texte : il commence par "ytsearch1:", jamais par un tiret.
        Ok(format!("ytsearch1:{text}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_a_normal_link() {
        assert_eq!(
            normalize_input("  https://www.youtube.com/watch?v=abc  ").unwrap(),
            "https://www.youtube.com/watch?v=abc"
        );
    }

    #[test]
    fn adds_https_when_missing() {
        assert_eq!(normalize_input("www.tiktok.com/@a/video/1").unwrap(), "https://www.tiktok.com/@a/video/1");
        assert_eq!(normalize_input("youtube.com/watch?v=abc").unwrap(), "https://youtube.com/watch?v=abc");
    }

    #[test]
    fn keywords_become_a_search() {
        assert_eq!(normalize_input("tove lo habits").unwrap(), "ytsearch1:tove lo habits");
    }

    #[test]
    fn option_lookalikes_stay_plain_text() {
        // Ça ne peut JAMAIS devenir une option de yt-dlp.
        assert_eq!(normalize_input("--exec calc.exe").unwrap(), "ytsearch1:--exec calc.exe");
        assert!(normalize_input("-o x").unwrap().starts_with("ytsearch1:"));
    }

    #[test]
    fn dangerous_schemes_are_refused() {
        assert!(normalize_input("file:///C:/Windows/system.ini").is_err());
        assert!(normalize_input("ftp://example.com/a.mp4").is_err());
    }

    #[test]
    fn empty_and_control_characters_are_refused() {
        assert!(normalize_input("   ").is_err());
        assert!(normalize_input("abc\ndef").is_err());
        assert!(normalize_input("a\0b").is_err());
        assert!(normalize_input(&"a".repeat(2001)).is_err());
    }
}
