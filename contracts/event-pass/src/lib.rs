#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env,
};

// --- Manejo de Errores Tipados ---
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PassError {
    AlreadyInitialized = 1,
    AlreadyHasPass = 2,
    PassNotFound = 3,
    PassAlreadyUsed = 4,
}

// --- Eventos Estructurados (Standard Soroban Events) ---
#[contractevent]
pub struct PassPurchased {
    #[topic]
    pub user: Address,
}

#[contractevent]
pub struct PassUsed {
    #[topic]
    pub user: Address,
}

// --- Claves de Almacenamiento ---
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Pass(Address),
}

// Estructura del estado del pase
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PassStatus {
    pub purchased: bool,
    pub used: bool,
}

// Configuración de extensión de vida útil (TTL en ledgers ~ 30 días)
const BUMP_AMOUNT: u32 = 518_400; // ~30 días
const BUMP_THRESHOLD: u32 = 172_800; // ~10 días

#[contract]
pub struct EventPassContract;

#[contractimpl]
impl EventPassContract {
    /// Constructor: Se ejecuta al desplegar e inicializa al Administrador.
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Compra o asigna un pase a una dirección de usuario.
    pub fn buy_pass(env: Env, user: Address) -> Result<(), PassError> {
        // Requiere la firma del usuario para confirmar la acción/pago
        user.require_auth();

        let key = DataKey::Pass(user.clone());

        // Verificar si la clave ya existe
        if env.storage().persistent().has(&key) {
            return Err(PassError::AlreadyHasPass);
        }

        let status = PassStatus {
            purchased: true,
            used: false,
        };

        // Almacenar el estado en Persistent Storage y asegurar su TTL
        env.storage().persistent().set(&key, &status);
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_AMOUNT);

        // Emitir evento
        PassPurchased { user }.publish(&env);

        Ok(())
    }

    /// Valida y consume el pase de un usuario (debe usarse solo una vez).
    pub fn use_pass(env: Env, user: Address) -> Result<(), PassError> {
        // Exige la firma del dueño del pase
        user.require_auth();

        let key = DataKey::Pass(user.clone());

        // 1. Verificar si el pase existe
        let mut status: PassStatus = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(PassError::PassNotFound)?;

        // 2. Verificar que no haya sido consumido previamente
        if status.used {
            return Err(PassError::PassAlreadyUsed);
        }

        // 3. Cambiar estado a usado y actualizar en el Ledger
        status.used = true;
        env.storage().persistent().set(&key, &status);
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_AMOUNT);

        // Emitir evento de entrada
        PassUsed { user }.publish(&env);

        Ok(())
    }

    /// Método de consulta rápida para verificar si la dirección tiene un pase activo y usable.
    pub fn check_pass(env: Env, user: Address) -> PassStatus {
        let key = DataKey::Pass(user);
        env.storage().persistent().get(&key).unwrap_or(PassStatus {
            purchased: false,
            used: false,
        })
    }
}


#[cfg(test)]
mod test;
