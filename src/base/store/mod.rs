use std::sync::Arc;

use super::web::{request::Request, response::Response};


pub struct DataStore {
    request_history: Vec<Request>,

    index_current_request: usize,
    current_request: Arc<Request>,
    last_response: Arc<Response>,
}

impl DataStore {
    pub fn init(requests: Vec<Request>) -> Self {
        let mut requests_to_use = requests.clone();

        if requests.len() == 0 {
            println!("DISPEROIU");
            requests_to_use = vec![Request::default()];
        }

        let current_request = Arc::new(requests_to_use[0].clone());
        let last_response = Arc::new(Response::default());
        println!("DISPEROIU 2");

        Self {
            request_history: requests_to_use,
            index_current_request: 0,
            current_request,
            last_response
        }
    }
    pub fn get_request(&self) -> Arc<Request> {
        self.current_request.clone()
    }

    pub fn get_requests(&self) -> &Vec<Request> {
        &self.request_history
    }

    pub fn request_ind(&self) -> usize {
        self.index_current_request
    }

    pub fn update_request(&mut self, request:Request) -> () {
        self.current_request = Arc::new(request.clone());
        self.save_request(request);
    }

    pub fn save_request(&mut self, request:Request) -> () {
        self.request_history[ self.index_current_request ] = (*self.current_request).clone();
    }

    pub fn goto_request(&mut self, index:usize) -> () {
        self.index_current_request = index;
        let req = self.request_history[ index ].clone();
        self.current_request = Arc::new(req);
    }

    pub fn goto_next_request(&mut self) -> () {
        self.index_current_request += 1;
        self.goto_request( self.index_current_request )
    }
    pub fn goto_prev_request(&mut self) -> () {
        self.index_current_request -= 1;
        self.goto_request( self.index_current_request )
    }

    pub fn add_request(&mut self, req: Request) -> usize {
        self.request_history.push(req);
        let ind = self.request_history.len() - 1;
        self.goto_request(ind);
        ind
    }


    // Response
    pub fn get_response(&self) -> Arc<Response> {
        self.last_response.clone()
    }
    pub fn update_response(&mut self, response: Response) -> () {
        self.last_response = Arc::new(response);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_and_point_to_current_request_corretly() {
        let mut data_store = DataStore::init(vec![]);
        assert_eq!(data_store.get_request().name, Request::default().name);
        assert_eq!(data_store.request_history.len(), 1);

        let mut req2 = Request::default();
        req2.name = String::from("Req2");
        data_store.add_request(req2);

        let mut req3 = Request::default();
        req3.name = String::from("Req3");
        data_store.add_request(req3);

        assert_eq!(data_store.request_history.len(), 3);
        assert_eq!(data_store.request_ind(), 2);
        assert_eq!(data_store.get_request().name, "Req3".to_string());
    }

    #[test]
    fn should_jump_and_goto_to_requests() {
        let mut req0 = Request::default();
        req0.set_name("Req0");

        let mut req1 = Request::default();
        req1.set_name("Req1");

        let mut req2 = Request::default();
        req2.set_name("Req2");

        let mut data_store = DataStore::init(vec![
            req0, req1, req2
        ]);

        // Init in first
        assert_eq!(data_store.request_ind(), 0);
        assert_eq!(data_store.get_request().name, "Req0");

        data_store.goto_request(2);
        assert_eq!(data_store.request_ind(), 2);
        assert_eq!(data_store.get_request().name, "Req2");

        data_store.goto_request(1);
        assert_eq!(data_store.request_ind(), 1);
        assert_eq!(data_store.get_request().name, "Req1");

        data_store.goto_next_request();
        assert_eq!(data_store.request_ind(), 2);
        assert_eq!(data_store.get_request().name, "Req2");

        data_store.goto_prev_request();
        assert_eq!(data_store.request_ind(), 1);
        assert_eq!(data_store.get_request().name, "Req1");
    }

    #[test]
    fn should_update_and_get_current_request() {
        let mut req = Request::default();
        let mut data_store = DataStore::init(vec![
            req.clone()
        ]);

        req.set_name("New name 1");
        assert_eq!(data_store.get_request().name, Request::default().name);

        data_store.update_request(req);
        assert_eq!(data_store.get_request().name, "New name 1".to_string());

        let mut req2 = Request::default();
        req2.set_name("New name 2");
        data_store.add_request(req2.clone());
        assert_eq!(data_store.get_request().name, "New name 2".to_string());

        req2.set_name("New name 2 after alter");

        assert_eq!(data_store.get_request().name, "New name 2".to_string());
        data_store.update_request(req2);
        assert_eq!(data_store.get_request().name, "New name 2 after alter".to_string());

        data_store.goto_request(0);
        assert_eq!(data_store.get_request().name, "New name 1".to_string());
    }
}
