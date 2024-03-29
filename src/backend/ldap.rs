use anyhow::anyhow;
use ldap3::result::Result;
use ldap3::{Ldap, LdapConnAsync, Scope, SearchEntry, SearchResult};
use leptos_reactive::use_context;
use std::sync::Arc;

#[cfg(feature = "ssr")]
pub async fn init(
    url: &str,
    base_dn: &str,
    attr: &str,
    bind: Option<(String, String)>,
) -> anyhow::Result<LdapIdent> {
    let l = LdapIdent::new(url, base_dn, attr, bind).await?;
    Ok(l)
}

#[cfg(feature = "ssr")]
pub async fn from_scope(cx: leptos::Scope) -> anyhow::Result<Arc<LdapIdent>> {
    Ok(use_context::<Arc<LdapIdent>>(cx).ok_or(anyhow::anyhow!("No ldap context found"))?)
}

#[derive(Clone)]
pub struct LdapIdent {
    ldap: Ldap,
    base_dn: String,
    attr: String,
}

#[derive(Clone)]
pub struct AccountInfo {
    pub uid: String,
    pub display_name: String,
}

impl LdapIdent {
    pub async fn new(
        url: &str,
        base_dn: &str,
        attr: &str,
        bind: Option<(String, String)>,
    ) -> Result<Self> {
        get_ldap(url, bind).await.map(|ldap| LdapIdent {
            ldap,
            base_dn: base_dn.to_string(),
            attr: attr.to_string(),
        })
    }
    pub async fn search(&self, uid: &str) -> Result<Vec<SearchEntry>> {
        Ok(self._search(&format!("{}*", uid)).await?)
    }
    pub async fn bind(&self, uid: &str, password: &str) -> anyhow::Result<AccountInfo> {
        let rs = self._search(&uid).await?;
        if rs.len() != 1 {
            return Err(anyhow!("Invalid username or password"));
        }
        let entry = rs[0].clone();
        let uid = entry
            .attrs
            .get(&self.attr)
            .unwrap()
            .get(0)
            .unwrap()
            .to_string();
        let display_name = match entry.attrs.get("displayName") {
            Some(v) => v.get(0).unwrap_or(&uid).to_string(),
            None => uid.clone(),
        };
        let ac = AccountInfo { uid, display_name };
        let mut ldap = self.ldap.clone();
        ldap.simple_bind(&rs.get(0).unwrap().dn, &password)
            .await?
            .success()?;
        Ok(ac)
    }
    async fn _search(&self, uid: &str) -> Result<Vec<SearchEntry>> {
        let filter = format!("(&({}={}))", self.attr, uid);
        let mut ldap = self.ldap.clone();
        let SearchResult(rs, _) = ldap
            .search(
                &self.base_dn,
                Scope::Subtree,
                &filter,
                vec!["uid", "displayName", "cn", "dn", &self.attr],
            )
            .await?;
        let mut r = Vec::new();
        for entry in rs {
            let entry = SearchEntry::construct(entry);
            r.push(entry);
        }
        Ok(r)
    }
    pub async fn all_accounts(&mut self) -> Result<Vec<AccountInfo>> {
        let SearchResult(rs, _) = self
            .ldap
            .search(
                &self.base_dn,
                Scope::Subtree,
                &format!("({}={})", self.attr, "*"),
                vec!["uid", "displayName", "cn", "dn", &self.attr],
            )
            .await?;

        Ok(rs
            .iter()
            .map(|entry| {
                let entry = SearchEntry::construct(entry.clone());
                let uid = entry
                    .attrs
                    .get(&self.attr)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .to_string();
                let display_name = match entry.attrs.get("displayName") {
                    Some(v) => v.get(0).unwrap_or(&uid).to_string(),
                    None => uid.clone(),
                };
                AccountInfo { uid, display_name }
            })
            .collect())
    }
}
// async fn search()

async fn get_ldap(url: &str, bind: Option<(String, String)>) -> Result<Ldap> {
    // set up ldap connection
    let (conn, mut ldap) = LdapConnAsync::new(url).await?;
    ldap3::drive!(conn);
    if let Some((dn, pw)) = bind {
        ldap.simple_bind(&dn, &pw).await?;
    }
    Ok(ldap)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        let mut ident = LdapIdent::new(&"ldap://127.0.0.1:1389", &"dc=example,dc=org", "cn", None)
            .await
            .unwrap();

        let rs = ident.search("user").await.unwrap();
        assert_eq!(4, rs.len());
        println!("{:?}", rs);

        let res = ident.bind("usera", "1111").await.unwrap();
        assert_eq!(true, res);

        let res = ident.bind("usera", "2222").await.unwrap();
        assert_eq!(false, res);

        let res = ident.bind("user-not-exist", "2222").await.unwrap();
        assert_eq!(false, res);
    }
}
