use crate::Graph;

impl Graph {
    pub fn add_dependency_package(
        &mut self,
        package: &str,
        version: &str,
    ) -> Result<(), String> {
        if let Some(v) = self.dep_package.get(package) {
            if v != version {
                return Err(format!(
                    "Version collision for package {}, current {} new {}",
                    package, v, version
                ));
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        }
    }
    pub fn remove_dependency_package(&mut self, package: &str) {
        self.dep_package.remove(&package.to_string());
    }

    pub fn get_new_id(&mut self) -> u64 {
        self.last_used_id = self.last_used_id + 1;
        // overflow check
        if self.last_used_id == 0 {
            panic!("All ids exhausted!!!");
        }

        return self.last_used_id;
    }
    pub fn to_ron(&self) -> Result<String, ()> {
        let pretty = ron::ser::PrettyConfig::new()
            .with_enumerate_arrays(true)
            .with_new_line("\n".to_string());

        if let Ok(s) = ron::ser::to_string_pretty(self, pretty) {
            return Ok(s);
        }

        return Err(());
    }
}
