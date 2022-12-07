use crate::data::accounts::Account;
use ldap3::result::Result;
use ldap3::{Ldap, LdapConnAsync, Scope, SearchEntry, SearchResult};

#[derive(Clone)]
pub struct LdapIdent {
    ldap: Ldap,
    base_dn: String,
    attr: String,
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
    pub async fn search(&mut self, uid: &str) -> Result<Vec<SearchEntry>> {
        Ok(self._search(&format!("{}*", uid)).await?)
    }
    pub async fn bind(&mut self, uid: &str, password: &str) -> Result<bool> {
        let rs = self._search(&uid).await?;
        if rs.len() != 1 {
            return Ok(false);
        }
        self.ldap
            .simple_bind(&rs.get(0).unwrap().dn, &password)
            .await?
            .success()
            .map_or(Ok(false), |_| Ok(true))
    }
    async fn _search(&mut self, uid: &str) -> Result<Vec<SearchEntry>> {
        let filter = format!("(&({}={}))", self.attr, uid);
        let SearchResult(rs, _) = self
            .ldap
            .search(
                &self.base_dn,
                Scope::Subtree,
                &filter,
                vec!["uid", "displayName", "cn", "dn"],
            )
            .await?;
        let mut r = Vec::new();
        for entry in rs {
            let entry = SearchEntry::construct(entry);
            r.push(entry);
        }
        Ok(r)
    }
    async fn sync_account(&mut self) -> Result<()> {
        let SearchResult(rs, _) = self
            .ldap
            .search(
                &self.base_dn,
                Scope::Subtree,
                &format!("({}={})", self.attr, "*"),
                vec!["uid", "displayName", "cn", "dn"],
            )
            .await?;
        for entry in rs {
            let entry = SearchEntry::construct(entry);
            let uid = entry.attrs.get("uid").unwrap().get(0).unwrap().to_string();
            let cn = entry.attrs.get("cn").unwrap().get(0).unwrap().to_string();
            let dn = entry.dn;
            let display_name = entry
                .attrs
                .get("displayName")
                .unwrap()
                .get(0)
                .unwrap()
                .to_string();
        }
        Ok(())
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
    use crate::data::ldap::LdapIdent;

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
