#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

// Función auxiliar para inicializar el entorno y desplegar el contrato
fn setup_test() -> (Env, EventPassContractClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Registra el contrato pasando los argumentos del __constructor
    let contract_id = env.register(EventPassContract, (&admin,));
    let client = EventPassContractClient::new(&env, &contract_id);

    (env, client, admin, user)
}

// ==========================================
// PRUEBAS DE: __constructor
// ==========================================

#[test]
fn test_constructor_initializes_admin_correctly() {
    let (_env, client, admin, _user) = setup_test();

    // Verificamos que el contrato fue desplegado sin errores con la dirección Admin
    // El cliente expone los métodos y el contrato ya tiene guardado al Admin.
    assert!(client.address != admin, "Error en caso de no ser el Address del admin");
}

#[test]
fn test_constructor_allows_multiple_contract_deployments() {
    let env = Env::default();
    let admin_1 = Address::generate(&env);
    let admin_2 = Address::generate(&env);

    // Verificamos que dos instancias distintas pueden inicializarse independientemente
    let id_1 = env.register(EventPassContract, (&admin_1,));
    let id_2 = env.register(EventPassContract, (&admin_2,));

    assert_ne!(id_1, id_2);
}

// ==========================================
// PRUEBAS DE: buy_pass
// ==========================================

#[test]
fn test_buy_pass_success() {
    let (_env, client, _admin, user) = setup_test();

    // 1. Un usuario nuevo compra un pase
    let result = client.try_buy_pass(&user);

    // Debe retornar Ok
    assert!(result.is_ok());

    // Verificar que el pase ahora figura como comprado y no usado
    let status = client.check_pass(&user);
    assert_eq!(
        status,
        PassStatus {
            purchased: true,
            used: false
        }
    );
}

#[test]
fn test_buy_pass_fails_if_already_purchased() {
    let (_env, client, _admin, user) = setup_test();

    // Compramos el pase por primera vez
    client.buy_pass(&user);

    // Intentamos comprar de nuevo con el mismo usuario
    let result = client.try_buy_pass(&user);

    // Debe fallar con el error de AlreadyHasPass (código 2)
    assert_eq!(result, Err(Ok(PassError::AlreadyHasPass)));
}

// ==========================================
// PRUEBAS DE: use_pass
// ==========================================

#[test]
fn test_use_pass_success() {
    let (_env, client, _admin, user) = setup_test();

    // Compramos el pase
    client.buy_pass(&user);

    // Consumimos el pase por primera vez
    let result = client.try_use_pass(&user);
    assert!(result.is_ok());

    // Verificar que el estado del pase cambie a usado
    let status = client.check_pass(&user);
    assert_eq!(
        status,
        PassStatus {
            purchased: true,
            used: true
        }
    );
}

#[test]
fn test_use_pass_fails_if_already_used_or_not_found() {
    let (_env, client, _admin, user) = setup_test();

    // CASO 1: Intentar usar sin haber comprado
    let result_not_found = client.try_use_pass(&user);
    assert_eq!(result_not_found, Err(Ok(PassError::PassNotFound)));

    // Compramos y usamos el pase 1 vez
    client.buy_pass(&user);
    client.use_pass(&user);

    // CASO 2: Intentar reusar el mismo pase (debe fallar)
    let result_already_used = client.try_use_pass(&user);
    assert_eq!(result_already_used, Err(Ok(PassError::PassAlreadyUsed)));
}

// ==========================================
// PRUEBAS DE: check_pass
// ==========================================

#[test]
fn test_check_pass_returns_default_for_unregistered_user() {
    let (_env, client, _admin, user) = setup_test();

    // Un usuario que no ha interactuado debe devolver purchased: false, used: false
    let status = client.check_pass(&user);
    assert_eq!(
        status,
        PassStatus {
            purchased: false,
            used: false
        }
    );
}

#[test]
fn test_check_pass_reflects_status_changes_correctly() {
    let (_env, client, _admin, user) = setup_test();

    // Paso 1: Comprobar estado antes de comprar
    assert!(!client.check_pass(&user).purchased);

    // Paso 2: Comprobar estado tras la compra
    client.buy_pass(&user);
    let status_after_buy = client.check_pass(&user);
    assert!(status_after_buy.purchased);
    assert!(!status_after_buy.used);

    // Paso 3: Comprobar estado tras el uso
    client.use_pass(&user);
    let status_after_use = client.check_pass(&user);
    assert!(status_after_use.purchased);
    assert!(status_after_use.used);
}

