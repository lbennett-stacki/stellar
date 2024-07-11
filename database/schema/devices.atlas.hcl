table "devices" {
  schema = schema.main

  column "id" {
    type = varchar(32)
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "user_id" {
    type = varchar(32)
  }
  foreign_key "user_fk" {
    columns     = [column.user_id]
    ref_columns = [table.users.column.id]
  }

  column "password" {
    type = varchar(255)
  }

  column "created_at" {
    type = date
  }

  column "updated_at" {
    type = date
    null = true
  }
}
