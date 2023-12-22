//use actix_web::{post, web, HttpResponse, Responder};
//use async_stripe::Event;
//use async_stripe::Webhook;
//
//
//
//
//let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//
//let customer = stripe::Customer::create(
//   &client,
//   stripe::CreateCustomer::new().email("customer@example.com")
//).await.unwrap();
//
//
//let plan = stripe::Plan::create(
//    &client,
//    stripe::CreatePlan::new()
//        .product("prod_J6a9q2yqk2z5NV".parse().unwrap())
//        .nickname("Legado Licence")
//        .interval(stripe::PlanInterval::Month)
//        .currency(stripe::Currency::BRL)
//        .amount(2000)
// ).await.unwrap();
//
//
// let subscription = stripe::Subscription::create(
//    &client,
//    stripe::CreateSubscription::new()
//        .customer(customer.id)
//        .items(vec![
//            stripe::CreateSubscriptionItem::new()
//                .plan(plan.id)
//        ])
// ).await.unwrap();
//
// match subscription {
//    Ok(_) => {
//        update_customer_status_in_database(&customer.id, "assinante").await;
//    },
//    Err(err) => {
//        println!("Erro ao criar a assinatura: {}", err);
//    }
// }
//
//
//
//
//
// #[post("/webhook")]
//async fn handle_webhook(payload: String, sig_header: String) -> impl Responder {
//   let sig_header = sig_header.split(',').collect::<Vec<&str>>();
//   let timestamp = sig_header[0].split('=').last().unwrap();
//   let sig = sig_header[1].split('=').last().unwrap();
//
//   let payload = Webhook::construct_event(payload, sig, timestamp).unwrap();
//
//   match &payload {
//       Event::CustomerSubscriptionDeleted(event) => {
//           update_customer_status_in_database(&event.customer, "cancelado").await;
//       },
//       _ => {},
//   }
//
//   HttpResponse::Ok().finish()
//}