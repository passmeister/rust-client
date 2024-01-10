use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  pass_api: Box<::apis::PassApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      pass_api: Box::new(::apis::PassApiClient::new(rc.clone())),
    }
  }

  pub fn pass_api(&self) -> &::apis::PassApi{
    self.pass_api.as_ref()
  }


}
